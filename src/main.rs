mod routes {
    pub mod user;
    pub mod health_check;
    pub mod auth;

    pub mod api {
        pub mod get_random_number;
    }
}

mod middlewares {
    pub mod simple_logger;
    pub mod api_key_validator;
    pub mod admin_validator;
    pub mod api_usage_logger;
}

mod models {
    pub mod api_usage;
}

use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");

    // Create PostgreSQL connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(middlewares::simple_logger::SimpleLogger)
            .app_data(web::Data::new(db_pool.clone()))
            .service(
                web::scope("/dashboard")
                    .service(
                        web::scope("/admin")
                            .wrap(middlewares::admin_validator::AdminValidator::new(db_pool.clone()))
                            .route("/users", web::get().to(routes::user::get_users))
                            .route("/users/{id}", web::get().to(routes::user::get_user_by_id))
                            .route("/users", web::post().to(routes::user::add_user))
                            .route("/users/{id}", web::delete().to(routes::user::delete_user))
                            .route("/users/{id}/{permission}", web::post().to(routes::user::change_permission)),
                    )
                    .route("/users/{id}/refresh_api_key", web::post().to(routes::user::refresh_api_key))
                    .route("/login", web::post().to(routes::auth::login))
                    .route("/register", web::post().to(routes::auth::register))
            )

            .service(
                web::scope("/api")
                    .wrap(middlewares::api_key_validator::ApiKeyValidator::new(db_pool.clone()))
                    .wrap(middlewares::api_usage_logger::ApiUsageLogger::new(db_pool.clone()))
                    .service(
                        web::scope("/v1")
                            .route("/get_random_number", web::get().to(routes::api::get_random_number::get_random_number)),
                    )
                    // TODO: Add GraphQL endpoint here
            )
            .route("/ping", web::get().to(routes::health_check::health_check))

    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}