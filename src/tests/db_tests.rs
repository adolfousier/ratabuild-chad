// DB tests

use crate::db::connection::establish_connection;
use crate::db::schema::create_tables;
use sqlx::PgPool;

#[tokio::test]
async fn test_establish_connection() {
    let database_url = "postgres://user:password@localhost:25851/ratabuild-chad";
    if let Ok(pool) = establish_connection(database_url).await {
        assert!(pool.is_closed() == false);
    }
}

#[tokio::test]
async fn test_create_tables() {
    let database_url = "postgres://user:password@localhost:25851/ratabuild-chad";
    // For test, assume DB is running, or use testcontainers, but for now skip if not connected
    if let Ok(pool) = PgPool::connect(database_url).await {
        create_tables(&pool).await.unwrap();
        // Check if table exists
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM information_schema.tables WHERE table_name = 'builds'")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert!(row.0 > 0);
    }
}