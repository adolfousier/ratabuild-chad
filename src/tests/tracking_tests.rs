// Tracking tests

use crate::db::connection::establish_connection;
use crate::tracking::logger::BuildLogger;

#[tokio::test]
async fn test_log_build() {
    let database_url = "postgres://user:password@localhost:25851/ratabuild-chad";
    if let Ok(logger) = BuildLogger::new(database_url).await {
        logger
            .log_build("/path/to/project", "rust", "/path/to/target", 1024)
            .await
            .unwrap();
        // Check if inserted
        let pool = establish_connection(database_url).await.unwrap();
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM builds")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert!(count.0 >= 1);
    }
}