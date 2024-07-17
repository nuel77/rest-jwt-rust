use crate::configuration::db::DatabasePool;
use crate::models::user_model::User;
use crate::models::user_token::UserToken;
use actix_web::{HttpRequest, web};
use jsonwebtoken::TokenData;
use regex::Regex;
use crate::constants;

pub fn get_secret_key() -> Vec<u8> {
    let secret_key = std::env::var("SECRET_KEY").expect("key not found");
    secret_key.as_bytes().to_vec()
}

pub fn decode_token(
    token: String,
    key: &[u8],
) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(
        &token,
        &jsonwebtoken::DecodingKey::from_secret(key),
        &jsonwebtoken::Validation::default(),
    )
}

pub fn verify_token(
    token_data: &TokenData<UserToken>,
    pool: &web::Data<DatabasePool>,
) -> Result<String, String> {
    if User::is_valid_login_session(&token_data.claims, &mut pool.get().unwrap()) {
        Ok(token_data.claims.email.to_string())
    } else {
        Err("Invalid token".to_string())
    }
}

pub fn validate_email(email: &str) -> bool {
    let email_regex: Regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    ).unwrap();
    email_regex.is_match(email)
}

pub fn extract_jwt_token(req: &HttpRequest) -> Option<TokenData<UserToken>> {
    let header = req.headers().get(constants::AUTHORIZATION_HEADER)?;
    let auth = header.to_str().ok()?;

    if !auth.to_uppercase().starts_with(constants::BEARER_PREFIX) {
        return None;
    };
    let token = auth[constants::BEARER_PREFIX.len()..auth.len()].trim();

    let token_data = decode_token(token.to_string(), &get_secret_key()).ok()?;
    Some(token_data)
}