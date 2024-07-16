use crate::configuration::db::DatabasePool;
use crate::models::user_model::{LoginDTO, LoginInfoDTO, User, UserInfoDTO};
use crate::services::errors::ServiceError;
use actix_web::web;
use crate::models::transaction_model::{Transaction, TransactionInfoDTO};
use crate::models::user_token::{TokenBodyResponse, UserToken};

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
) -> Result<TokenBodyResponse, ServiceError> {
    //check if already logged in and valid token
    let login_info = User::login(user, &mut db_pool.get().unwrap()).map_err(|e| ServiceError::InternalServerError {
        error_message: e.to_string(),
    })?;

    //check if session is empty
    if login_info.session.is_empty() {
        return Err(ServiceError::InternalServerError { error_message: "Session is empty".to_string() });
    }

    //create a json token for this user
    Ok(TokenBodyResponse {
        token: UserToken::generate_token(&login_info),
        token_type: "Bearer".to_string(),
    })
}