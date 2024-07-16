use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use log::debug;
use serde::{Deserialize, Serialize};
use std::env;

use crate::models::user_model::LoginInfoDTO;
use crate::utils::get_secret_key;

static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub email: String,
    pub login_session: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

impl UserToken {
    pub fn generate_token(login: &LoginInfoDTO) -> String {
        dotenv::dotenv().expect("Failed to read .env file");
        let max_age= ONE_WEEK;
        debug!("Token Max Age: {}", max_age);

        let now = Utc::now().timestamp();
        let payload = UserToken {
            iat: now,
            exp: now + max_age,
            email: login.email.clone(),
            login_session: login.session.clone(),
        };

        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(&get_secret_key()),
        )
        .unwrap()
    }
}
