use actix_web::{HttpResponse, web};
use crate::configuration::db::DatabasePool;
use crate::constants::{MESSAGE_EMPTY, MESSAGE_OK};
use crate::controllers::types::ResponseBody;
use crate::models::transaction_model::TransactionInfoDTO;
use crate::services::errors::ServiceError;
use crate::services::transaction_service;

pub async fn transfer(
    tx: web::Json<TransactionInfoDTO>,
    pool: web::Data<DatabasePool>,
) -> Result<HttpResponse, ServiceError> {
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