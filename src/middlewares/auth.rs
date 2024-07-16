use std::future::{ready, Ready};

use crate::configuration::db::DatabasePool;
use crate::controllers::types::ResponseBody;
use crate::{constants, utils};
use actix_web::body::EitherBody;
use actix_web::web::head;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpResponse,
};
use futures::future::LocalBoxFuture;
use log::error;
use crate::utils::get_secret_key;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct JWTAuthentication;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for JWTAuthentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = JWTAuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JWTAuthenticationMiddleware { service }))
    }
}

pub struct JWTAuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JWTAuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut is_authorized = false;

        //check if jwt token needs to be checked
        for route in constants::UNPROTECTED_ROUTES {
            if req.path().starts_with(route) {
                is_authorized = true;
                break;
            }
        }

        if !is_authorized {
            if let Some(pool) = req.app_data::<web::Data<DatabasePool>>() {
                if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION_HEADER) {
                    if let Ok(authen_str) = authen_header.to_str() {
                        if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer") {
                            let token = authen_str[6..authen_str.len()].trim();
                            if let Ok(token_data) = utils::decode_token(token.to_string(), &get_secret_key()) {
                                if utils::verify_token(&token_data, pool).is_ok() {
                                    is_authorized = true;
                                } else {
                                    error!("Invalid token");
                                }
                            }
                        }
                    }
                }
            }
        }

        if !is_authorized {
            let (request, _pl) = req.into_parts();
            let response = HttpResponse::Unauthorized()
                .json(ResponseBody::new(
                    constants::MESSAGE_INVALID_TOKEN,
                    constants::MESSAGE_EMPTY,
                ))
                .map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let fut = self.service.call(req);
        Box::pin(async move { fut.await.map(ServiceResponse::map_into_left_body) })
    }
}
