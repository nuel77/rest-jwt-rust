use crate::models::user_token::UserToken;
use crate::schema::users::dsl::users;
use crate::schema::users::{email, id};
use anyhow::anyhow;
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub balance: i32,
}

#[derive(Insertable, Selectable, Serialize, Deserialize, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserDTO {
    pub email: String,
    pub balance: i32,
    pub password: String,
}

#[derive(Insertable, Selectable, Serialize, Deserialize, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserInfoDTO {
    pub email: String,
    pub balance: i32,
}

#[derive(Insertable, Selectable, Serialize, Deserialize, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

pub struct LoginInfoDTO {
    pub email: String,
    pub session: String,
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
    pub fn register(who: LoginDTO, connection: &mut PgConnection) -> anyhow::Result<()> {
        if let Ok(_) = Self::find_user_by_email(&who.email, connection) {
            return Err(anyhow!("User already registered!"));
        }
        let mut user_dto: UserDTO = who.into();
        // hash password
        user_dto.password = hash(user_dto.password, DEFAULT_COST).map_err(|e| anyhow!(e))?;
        // add bonus balance to newly registered user
        user_dto.balance = 10;
        diesel::insert_into(users)
            .values(user_dto)
            .execute(connection)
            .map_err(|e| anyhow!(e))?;
        Ok(())
    }

    pub fn login(who: LoginDTO, conn: &mut PgConnection) -> anyhow::Result<LoginInfoDTO> {
        let Ok(user) = Self::find_user_by_email(&who.email, conn) else {
            return Err(anyhow!("unknown user"));
        };
        if !user.password.eq(&hash(who.password, DEFAULT_COST)?) {
            return Err(anyhow!("password does not match"));
        }
        let session = User::generate_random_session();

        Ok(LoginInfoDTO {
            email: user.email,
            session,
        })
    }
    pub fn query_all(_page: i64, conn: &mut PgConnection) -> QueryResult<Vec<UserInfoDTO>> {
        users.limit(100).select(UserInfoDTO::as_select()).load(conn)
    }
    pub fn find_user_by_email(email_id: &str, conn: &mut PgConnection) -> QueryResult<User> {
        users
            .filter(email.eq(email_id))
            .select(User::as_select())
            .first(conn)
    }

    pub fn is_valid_login_session(token: &UserToken, conn: &mut PgConnection) -> bool {
        true
    }
    pub fn generate_random_session() -> String {
        Uuid::new_v4().to_string()
    }
}
