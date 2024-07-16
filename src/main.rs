mod configuration;
mod constants;
mod controllers;
mod middlewares;
mod models;
mod schema;
mod services;
mod utils;

use crate::configuration::types::UserClaim;
use actix_cors::Cors;
use actix_jwt_auth_middleware::use_jwt::UseJWTOnApp;
use actix_jwt_auth_middleware::{Authority, TokenSigner};
use actix_web::{http, web, App, HttpServer};
use diesel::prelude::*;
use ed25519_compact::KeyPair;
use jwt_compact::alg::Ed25519;
use log::info;
use std::env;
use crate::utils::get_secret_key;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    env_logger::init();
    //check if secret key is present
    let _ = get_secret_key();

    let db_host = env::var("DATABASE_URL").expect("DATABASE_URL not found.");
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    let pool = configuration::db::create_db_pool(&db_host);
    configuration::db::run_migration(&mut pool.get().unwrap());

    let KeyPair { pk, sk } = KeyPair::generate();
    info!("starting server on {}:{}", host, port);

    HttpServer::new(move || {
        let authority = Authority::<UserClaim, Ed25519, _, _>::new()
            .refresh_authorizer(|| async move { Ok(()) })
            .token_signer(Some(
                TokenSigner::new()
                    .signing_key(sk.clone())
                    .algorithm(Ed25519)
                    .build()
                    .expect(""),
            ))
            .verifying_key(pk)
            .build()
            .expect("cannot create jwt authority");
        App::new()
            .wrap(
                Cors::default() // allowed_origin return access-control-allow-origin: * by default
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_origin("http://localhost:3000")
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .app_data(web::Data::new(pool.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .wrap(middlewares::auth::JWTAuthentication)
            .configure(configuration::routes::configure_routes)
    })
    .bind((host, port.parse().expect("invalid port")))?
    .run()
    .await
}
