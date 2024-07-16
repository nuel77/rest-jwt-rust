use crate::configuration::db::DatabasePool;
use crate::models::user_model::{LoginDTO, LoginInfoDTO, User, UserInfoDTO};
use crate::services::errors::ServiceError;
use actix_web::web;

pub fn register(user: LoginDTO, db_pool: &web::Data<DatabasePool>) -> Result<(), ServiceError> {
    User::register(user, &mut db_pool.get().unwrap()).map_err(|e| {
        ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
    })
}

pub fn query_all(
    page: i64,
    db_pool: &web::Data<DatabasePool>,
) -> Result<Vec<UserInfoDTO>, ServiceError> {
    User::query_all(page, &mut db_pool.get().unwrap()).map_err(|e| {
        ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
    })
}

pub fn login(
    user: LoginDTO,
    db_pool: &web::Data<DatabasePool>,
) -> Result<LoginInfoDTO, ServiceError> {
    User::login(user, &mut db_pool.get().unwrap()).map_err(|e| ServiceError::InternalServerError {
        error_message: e.to_string(),
    })
}
