use crate::controllers::user_controller;
use actix_web::{get, HttpResponse, web};

#[get("/ping")]
async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong!".to_string())
}
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(ping)
        .service(
            web::scope("/auth")
                .service(web::resource("/register").route(web::get().to(user_controller::register))),
        );
}



