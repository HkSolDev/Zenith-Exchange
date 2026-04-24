use sqlx::postgres::PgPoolOptions;
pub use sqlx::PgPool;
use std::env;

pub type DbPool = PgPool;


/// Initializes the PostgreSQL connection pool.
/// This should be called once by each microservice at startup.
pub async fn init_db_pool() -> Result<DbPool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    Ok(pool)
}
