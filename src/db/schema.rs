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


