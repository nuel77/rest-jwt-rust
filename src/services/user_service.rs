use crate::configuration::db::DatabasePool;
use crate::models::user_model::{User, UserDTO};
use crate::services::errors::ServiceError;
use actix_web::web;

pub fn register(user: UserDTO, db_pool: &web::Data<DatabasePool>) -> Result<(), ServiceError> {
    User::register(user, &mut db_pool.get().unwrap()).map_err(|e| {
        ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
    })
}
