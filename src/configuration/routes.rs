use crate::controllers::user_controller;
use actix_web::{get, web, HttpResponse};

#[get("/ping")]
async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong!".to_string())
}
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(ping)
        .service(
            web::scope("/auth")
                .service(
                    web::resource("/register").route(web::post().to(user_controller::register)),
                )
                .service(
                    web::resource("/login").route(web::post().to(user_controller::login)),
                ),
        )
        .service(
            web::scope("/users")
                .service(web::resource("/").route(web::get().to(user_controller::query_all))),
        );
}
