// Database connection management

use sqlx::PgPool;

pub async fn establish_connection(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(database_url).await
}


