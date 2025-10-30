// Database schema definitions

use sqlx::PgPool;

pub async fn create_tables(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS builds (
            id SERIAL PRIMARY KEY,
            project_path TEXT NOT NULL,
            language TEXT NOT NULL,
            build_time TIMESTAMPTZ DEFAULT NOW(),
            artifact_path TEXT NOT NULL,
            size_bytes BIGINT
        )",
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_old_artifact_paths(pool: &PgPool, retention_days: u32) -> Result<Vec<String>, sqlx::Error> {
    let artifacts = sqlx::query_as::<_, (String,)>(
        "SELECT DISTINCT artifact_path FROM builds WHERE build_time < NOW() - INTERVAL '1 day' * $1"
    )
    .bind(retention_days as i32)
    .fetch_all(pool)
    .await?;

    Ok(artifacts.into_iter().map(|(path,)| path).collect())
}

pub async fn delete_old_builds_from_db(pool: &PgPool, retention_days: u32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM builds WHERE build_time < NOW() - INTERVAL '1 day' * $1"
    )
    .bind(retention_days as i32)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}


