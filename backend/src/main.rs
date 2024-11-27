mod routes;
mod middlewares;
mod models;
mod utils;
mod config;

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

fn configure_routes(
    cfg: &mut web::ServiceConfig,
    db_pool: sqlx::postgres::PgPool,
    redis_client: redis::Client,
) {
    cfg
        .route("/login", web::post().to(routes::auth::login))
        .route("/register", web::post().to(routes::auth::register))
        .service(
            web::scope("/dashboard")
                .wrap(middlewares::jwt_validator::JwtValidator)
                .service(
                    web::scope("/admin")
                        .wrap(middlewares::admin_validator::AdminValidator::new(db_pool.clone()))
                        .configure(routes::user::configure_user_routes)
                )
                .route("/users/refresh_api_key", web::post().to(routes::user::refresh_api_key))
                .route("/get_api_key_usage/{size}", web::get().to(routes::user::get_api_key_usage))
                .route("/verify", web::get().to(routes::auth::verify))
        )

        .service(
            web::scope("/api")
                .wrap(middlewares::api_key_validator::ApiKeyValidator::new(db_pool.clone()))
                .wrap(middlewares::api_usage_logger::ApiUsageLogger::new(db_pool.clone()))
                .service(
                    web::scope("/v1")
                        .wrap(middlewares::rate_limiter::RateLimiter::new(redis_client.clone(), 5, std::time::Duration::from_secs(60)))
                        .route("/get_random_number", web::get().to(routes::api::v1::get_random_number::get_random_number)),
                )
                .service(
                    web::resource("/graphql")
                        .route(web::post().to(routes::api::graphql::setup::graphql_handler))
                )
        )
        .route("/playground", web::get().to(routes::api::graphql::setup::graphql_playground))
        .route("/ping", web::get().to(routes::health_check::health_check));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");

    // Create PostgreSQL connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = config::postgresql::create_db_pool(&database_url).await;

    // Create Redis connection pool
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis_client = config::redis::create_redis_client(&redis_url);

    // Create GraphQL schema
    let schema = routes::api::graphql::schema::create_schema();

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000") // Allow requests from your frontend
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]) // Allow specific HTTP methods
            .allowed_headers(vec!["Content-Type", "Authorization"]) // Allow specific headers
            .allow_any_header()
            .max_age(3600);

        let db_pool_clone = db_pool.clone();
        let redis_client_clone = redis_client.clone();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(schema.clone()))
            .configure(move |cfg| configure_routes(cfg, db_pool_clone.clone(), redis_client_clone.clone()))

    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
