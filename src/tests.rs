#[cfg(test)]
mod tests {
    use crate::models::user_model::LoginDTO;
    use crate::services::user_service;
    use crate::{configuration, middlewares};
    use actix_cors::Cors;
    use actix_web::body::to_bytes;
    use actix_web::{http, web, App};
    use diesel::r2d2::ConnectionManager;
    use diesel::{r2d2, PgConnection};
    use serde_json::json;
    use testcontainers::core::wait::HealthWaitStrategy;
    use testcontainers::core::WaitFor;
    use testcontainers::runners::AsyncRunner;
    use testcontainers::{GenericImage, ImageExt};

    pub type DatabasePool = r2d2::Pool<ConnectionManager<PgConnection>>;

    #[actix_web::test]
    async fn test_mock_transfer() {
        let container = GenericImage::new("postgres", "14")
            .with_wait_for(WaitFor::message_on_stderr(
                "database system is ready to accept connections",
            ))
            .with_env_var("POSTGRES_USER", "postgres")
            .with_env_var("POSTGRES_PASSWORD", "postgres")
            .with_env_var("POSTGRES_DB", "postgres")
            .start()
            .await
            .expect("cannot start container");

        let db_host = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            container.get_host_port_ipv4(5432).await.unwrap()
        );
        let port = "8080";
        let host = "0.0.0.0";

        let pool = configuration::db::create_db_pool(&db_host);
        configuration::db::run_migration(&mut pool.get().unwrap());

        let app = actix_web::test::init_service(
            App::new()
                .wrap(
                    Cors::default() // allowed_origin return access-control-allow-origin: * by default
                        .send_wildcard()
                        .allowed_header(http::header::CONTENT_TYPE)
                        .max_age(3600),
                )
                .app_data(web::Data::new(pool.clone()))
                .wrap(actix_web::middleware::Logger::default())
                .wrap(middlewares::auth::JWTAuthentication)
                .configure(configuration::routes::configure_routes),
        )
            .await;

        let login = json!({
            "email": "alicia@gmail.com",
            "password": "passpass"
        });

        let resp = actix_web::test::TestRequest::post()
            .uri("/auth/register")
            .insert_header(http::header::ContentType::json())
            .set_payload(login.to_string())
            .send_request(&app)
            .await;

        let login = json!({
            "email": "bobina@gmail.com",
            "password": "passpass"
        });

        let resp = actix_web::test::TestRequest::post()
            .uri("/auth/register")
            .insert_header(http::header::ContentType::json())
            .set_payload(login.to_string())
            .send_request(&app)
            .await;

        assert_eq!(resp.status(), 200);

        //login with the same credentials
        let login_dto = LoginDTO {
            email: "alicia@gmail.com".to_string(),
            password: "passpass".to_string(),
        };

        let Ok(jwt) = user_service::login(login_dto, &web::Data::new(pool.clone())) else {
            panic!("login failed");
        };
        let session = jwt.token;

        //try to transfer
        let transfer = json!({
            "from_user": "alicia@gmail.com",
            "to_user":"bobina@gmail.com",
            "amount": 5
        });

        let resp = actix_web::test::TestRequest::post()
            .uri("/transfer/create")
            .insert_header((http::header::AUTHORIZATION, format!("Bearer {:}", session)))
            .insert_header(http::header::ContentType::json())
            .set_payload(transfer.to_string())
            .send_request(&app)
            .await;

        assert_eq!(resp.status(), 200);

        //check that reverse is not possible with alicia's token
        let transfer = json!({
            "to_user": "alicia@gmail.com",
            "from_user":"bobina@gmail.com",
            "amount": 5
        });

        let resp = actix_web::test::TestRequest::post()
            .uri("/transfer/create")
            .insert_header((http::header::AUTHORIZATION, format!("Bearer {:}", session)))
            .insert_header(http::header::ContentType::json())
            .set_payload(transfer.to_string())
            .send_request(&app)
            .await;
        assert_ne!(resp.status(), 200);

        //check that transfers above balance are not possible
        //try to transfer
        let transfer = json!({
            "from_user": "alicia@gmail.com",
            "to_user":"bobina@gmail.com",
            "amount": 25
        });
        let resp = actix_web::test::TestRequest::post()
            .uri("/transfer/create")
            .insert_header((http::header::AUTHORIZATION, format!("Bearer {:}", session)))
            .insert_header(http::header::ContentType::json())
            .set_payload(transfer.to_string())
            .send_request(&app)
            .await;
        assert_ne!(resp.status(), 200);
    }
}
