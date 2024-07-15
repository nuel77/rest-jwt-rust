use crate::configuration::db::DatabasePool;
use crate::constants::{MESSAGE_EMPTY, MESSAGE_OK};
use crate::controllers::types::ResponseBody;
use crate::models::user_model::UserDTO;
use crate::services;
use crate::services::errors::ServiceError;
use crate::services::user_service;
use actix_web::{web, HttpResponse};

pub async fn register(
    user: web::Json<UserDTO>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, ServiceError> {
    // validate json
    match user_service::register(user.0, &pool) {
        Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, MESSAGE_EMPTY))),
        Err(e) => Err(e),
    }
}
