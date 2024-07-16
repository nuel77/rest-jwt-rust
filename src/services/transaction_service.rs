use crate::configuration::db::DatabasePool;
use crate::models::transaction_model::{Transaction, TransactionInfoDTO};
use crate::models::user_model::User;
use crate::services::errors::ServiceError;
use actix_web::web;

pub fn transfer(
    tx: &TransactionInfoDTO,
    db_pool: &web::Data<DatabasePool>,
) -> Result<(), ServiceError> {
    //transfer the amount
    let tx = User::try_transfer(
        &tx.from_user,
        &tx.to_user,
        tx.amount,
        &mut db_pool.get().unwrap(),
    )
    .map_err(|e| ServiceError::BadRequest {
        error_message: e.to_string(),
    })?;
    //add the transaction to the database
    Transaction::add_transfer_history(&tx, &mut db_pool.get().unwrap()).map_err(|e| {
        ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
    })?;
    Ok(())
}

pub fn query_all(
    page: i64,
    db_pool: &web::Data<DatabasePool>,
) -> Result<Vec<Transaction>, ServiceError> {
    Transaction::query_all(page, &mut db_pool.get().unwrap()).map_err(|e| {
        ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
    })
}
