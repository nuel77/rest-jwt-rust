use crate::configuration::db::DatabasePool;
use crate::constants::{MESSAGE_EMPTY, MESSAGE_OK};
use crate::controllers::types::ResponseBody;
use crate::models::user_model::LoginDTO;
use crate::services::errors::ServiceError;
use crate::services::user_service;
use actix_web::{web, HttpResponse};

pub async fn register(
    user: web::Json<LoginDTO>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, ServiceError> {
    // validate json
    match user_service::register(user.0, &pool) {
        Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, MESSAGE_EMPTY))),
        Err(e) => Err(e),
    }
}

pub async fn query_all(pool: web::Data<DatabasePool>) -> Result<HttpResponse, ServiceError> {
    match user_service::query_all(0, &pool) {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, message))),
        Err(e) => Err(e),
    }
}

pub async fn login(
    user: web::Json<LoginDTO>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, ServiceError> {
    match user_service::login(user.0, &pool) {
        Ok(token) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, token))),
        Err(e) => Err(e),
    }
}
