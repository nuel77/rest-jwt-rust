use crate::constants::email_regex;
use crate::models::transaction_model::TransactionDTO;
use crate::models::user_token::UserToken;
use crate::schema::users::dsl::users;
use crate::schema::users::{balance, email, id, session_token};
use anyhow::anyhow;
use bcrypt::{hash, verify, DEFAULT_COST};
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
    pub session_token: String,
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
        //check if valid email
        if !email_regex.is_match(&who.email) {
            return Err(anyhow!("invalid email provided!"));
        }
        //check for empty password
        if who.password.is_empty() {
            return Err(anyhow!("password cannot be empty"));
        }
        //check if already exists
        if Self::find_user_by_email(&who.email, connection).is_ok() {
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

    pub fn login(who: &LoginDTO, conn: &mut PgConnection) -> anyhow::Result<LoginInfoDTO> {
        let Ok(user) = Self::find_user_by_email(&who.email, conn) else {
            return Err(anyhow!("unknown user"));
        };
        if !verify(&who.password, &user.password).unwrap() {
            return Err(anyhow!("password does not match"));
        }
        let session = User::generate_random_session();
        User::set_session_token(user.id, &session, conn)?;
        Ok(LoginInfoDTO {
            email: user.email,
            session,
        })
    }
    pub fn query_all(page: i64, conn: &mut PgConnection) -> QueryResult<Vec<UserInfoDTO>> {
        users
            .limit(10)
            .offset(page * 10)
            .select(UserInfoDTO::as_select())
            .load(conn)
    }
    pub fn find_user_by_email(email_id: &str, conn: &mut PgConnection) -> QueryResult<User> {
        users
            .filter(email.eq(email_id))
            .select(User::as_select())
            .first(conn)
    }

    pub fn set_session_token(
        user_id: i32,
        session: &str,
        conn: &mut PgConnection,
    ) -> QueryResult<usize> {
        diesel::update(users.filter(id.eq(user_id)))
            .set(session_token.eq(session))
            .execute(conn)
    }

    pub fn is_valid_login_session(token: &UserToken, conn: &mut PgConnection) -> bool {
        let user = Self::find_user_by_email(&token.email, conn);
        if let Ok(user) = user {
            return user.email.eq(&token.email)
                && user.session_token.eq(&token.login_session)
                && token.exp > chrono::Utc::now().timestamp();
        }
        false
    }

    // Transfer money from one user to another
    pub fn try_transfer(
        from: &str,
        to: &str,
        amount: i32,
        conn: &mut PgConnection,
    ) -> anyhow::Result<TransactionDTO> {
        conn.build_transaction().run(|conn| {
            let from_user = Self::find_user_by_email(from, conn)?;
            let to_user = Self::find_user_by_email(to, conn)?;
            if from_user.balance < amount {
                return Err(anyhow!("transaction failed: insufficient balance"));
            }
            Self::set_balance(from_user.id, from_user.balance - amount, conn)?;
            Self::set_balance(to_user.id, to_user.balance + amount, conn)?;
            Ok(TransactionDTO {
                from_user: from_user.id,
                to_user: to_user.id,
                amount,
            })
        })
    }
    pub fn set_balance(who: i32, amount: i32, conn: &mut PgConnection) -> QueryResult<usize> {
        diesel::update(users.filter(id.eq(who)))
            .set(balance.eq(amount))
            .execute(conn)
    }

    pub fn generate_random_session() -> String {
        Uuid::new_v4().to_string()
    }
}
