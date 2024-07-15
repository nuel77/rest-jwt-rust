use crate::schema::users::dsl::users;
use crate::schema::users::email;
use anyhow::anyhow;
use diesel::prelude::*;
use diesel::sql_types::Uuid;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub balance: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserDTO {
    pub email: String,
    pub password: String,
    pub balance: i32,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

impl From<LoginDTO> for UserDTO {
    fn from(value: LoginDTO) -> Self {
        Self {
            email: value.email,
            password: value.password,
            balance: 0,
        }
    }
}
impl User {
    pub fn register(user: UserDTO, connection: &mut PgConnection) -> anyhow::Result<()> {
        // check if already registered
        //
        diesel::insert_into(users)
            .values(user)
            .execute(connection)
            .map_err(|e| anyhow!(e))?;
        Ok(())
    }

    pub fn find_user_by_email(email_id: &str, conn: &mut PgConnection) -> QueryResult<User> {
        users
            .filter(email.eq(email_id))
            .select(User::as_select())
            .first(conn)
    }

    pub fn generate_random_session() -> String {
        //todo!
        unimplemented!()
    }
}
