use crate::configuration::db::DatabasePool;
use crate::models::user_model::User;
use crate::models::user_token::UserToken;
use actix_web::web;
use jsonwebtoken::TokenData;

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
