use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn create_db_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to create pool")
}
