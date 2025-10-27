// Build logging functionality

use crate::db::connection::establish_connection;
use crate::db::schema::create_tables;
use sqlx::{PgPool, Row};

#[derive(Clone)]
pub struct BuildLogger {
    pub pool: PgPool,
}

impl BuildLogger {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = establish_connection(database_url).await?;
        create_tables(&pool).await?;
        Ok(BuildLogger { pool })
    }

    pub async fn log_build(
        &self,
        project_path: &str,
        language: &str,
        artifact_path: &str,
        size: u64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO builds (project_path, language, artifact_path, size_bytes) VALUES ($1, $2, $3, $4)",
        )
        .bind(project_path)
        .bind(language)
        .bind(artifact_path)
        .bind(size as i64)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn load_artifacts(&self) -> Result<Vec<String>, sqlx::Error> {
        let rows = sqlx::query("SELECT DISTINCT artifact_path FROM builds ORDER BY artifact_path")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(|row| row.get("artifact_path")).collect())
    }

    pub async fn load_history(&self) -> Result<Vec<String>, sqlx::Error> {
        let rows = sqlx::query("SELECT project_path, language, COUNT(*) as count FROM builds GROUP BY project_path, language ORDER BY project_path")
            .fetch_all(&self.pool)
            .await?;
        let history: Vec<String> = rows.into_iter().map(|row| {
            let project: String = row.get("project_path");
            let language: String = row.get("language");
            let count: i64 = row.get("count");
            format!("{} ({}) - {} builds", project, language, count)
        }).collect();
        Ok(history)
    }
}


