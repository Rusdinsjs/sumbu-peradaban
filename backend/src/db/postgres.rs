use sqlx::postgres::PgPool;
use tracing::info;

pub async fn init_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    info!("Connecting to PostgreSQL...");
    let pool = PgPool::connect(database_url).await?;
    info!("PostgreSQL connected successfully");
    Ok(pool)
}
