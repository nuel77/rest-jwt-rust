use std::future::{ready, Ready};

use crate::configuration::db::DatabasePool;
use crate::controllers::types::ResponseBody;
use crate::utils::{get_secret_key, verify_token};
use crate::{constants, utils};
use actix_web::body::EitherBody;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpResponse,
};
use futures::future::LocalBoxFuture;

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

        // check if token is valid
        if !is_authorized && self.check_if_token_valid(&req) {
            is_authorized = true;
        }

        if !is_authorized {
            let res = self.create_response(req, constants::MESSAGE_INVALID_TOKEN);
            return Box::pin(async { Ok(res) });
        }

        // if user tries to log in with an existing valid token
        if is_authorized && req.path().starts_with(constants::LOGIN_ROUTE) {
            let res = self.create_response(req, constants::MESSAGE_ALREADY_LOGGED_IN);
            return Box::pin(async { Ok(res) });
        }
        let fut = self.service.call(req);
        Box::pin(async move { fut.await.map(ServiceResponse::map_into_left_body) })
    }
}

impl<B, S> JWTAuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    pub fn check_if_token_valid(&self, req: &ServiceRequest) -> bool {
        let Some(pool) = req.app_data::<web::Data<DatabasePool>>() else {
            return false;
        };
        let Some(header) = req.headers().get(constants::AUTHORIZATION_HEADER) else {
            return false;
        };
        let Ok(auth) = header.to_str() else {
            return false;
        };

        if !auth.to_uppercase().starts_with(constants::BEARER_PREFIX) {
            return false;
        }
        let token = auth[constants::BEARER_PREFIX.len()..auth.len()].trim();

        let Ok(token_data) = utils::decode_token(token.to_string(), &get_secret_key()) else {
            return false;
        };

        if verify_token(&token_data, pool).is_err() {
            return false;
        }
        true
    }

    pub fn create_response(
        &self,
        req: ServiceRequest,
        message: &str,
    ) -> ServiceResponse<EitherBody<B>> {
        let (request, _pl) = req.into_parts();
        let response = HttpResponse::Unauthorized()
            .json(ResponseBody::new(message, constants::MESSAGE_EMPTY))
            .map_into_right_body();

        ServiceResponse::new(request, response)
    }
}
