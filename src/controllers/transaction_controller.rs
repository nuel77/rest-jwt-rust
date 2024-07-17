use crate::configuration::db::DatabasePool;
use crate::constants::{MESSAGE_EMPTY, MESSAGE_OK};
use crate::controllers::types::ResponseBody;
use crate::models::transaction_model::TransactionInfoDTO;
use crate::services::errors::ServiceError;
use crate::services::transaction_service;
use actix_web::{web, HttpResponse, HttpRequest};
use crate::utils::extract_jwt_token;

pub async fn transfer(
    rq: HttpRequest,
    tx: web::Json<TransactionInfoDTO>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, ServiceError> {
    // this route is only accessible is token is valid; so unwrap is safe
    let token = extract_jwt_token(&rq).unwrap();
    if token.claims.email != tx.0.from_user {
        return Err(ServiceError::Unauthorized {
            error_message: "Unauthorized to make this request".to_string(),
        });
    };
    match transaction_service::transfer(&tx.0, &pool) {
        Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, MESSAGE_EMPTY))),
        Err(e) => Err(e),
    }
}

pub async fn query_all(pool: web::Data<DatabasePool>) -> Result<HttpResponse, ServiceError> {
    match transaction_service::query_all(0, &pool) {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, message))),
        Err(e) => Err(e),
    }
}
