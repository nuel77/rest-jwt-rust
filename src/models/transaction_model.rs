use crate::schema::transactions;
use anyhow::anyhow;
use diesel::{
    Insertable, PgConnection, QueryDsl, QueryResult, Queryable, RunQueryDsl, Selectable,
    SelectableHelper,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Transaction {
    pub id: i32,
    pub from_user: i32,
    pub to_user: i32,
    pub amount: i32,
}
#[derive(Queryable, Insertable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TransactionDTO {
    pub from_user: i32,
    pub to_user: i32,
    pub amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionInfoDTO {
    pub from_user: String,
    pub to_user: String,
    pub amount: i32,
}

impl Transaction {
    pub fn add_transfer_history(
        transaction: &TransactionDTO,
        conn: &mut PgConnection,
    ) -> anyhow::Result<()> {
        diesel::insert_into(transactions::table)
            .values(transaction)
            .execute(conn)
            .map_err(|e| anyhow!(e))?;
        Ok(())
    }

    pub fn query_all(page: i64, conn: &mut PgConnection) -> QueryResult<Vec<Transaction>> {
        transactions::table
            .limit(10)
            .offset(page * 10)
            .select(Transaction::as_select())
            .load(conn)
    }
}
