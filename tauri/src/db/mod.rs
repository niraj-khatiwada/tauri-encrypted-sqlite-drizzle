pub mod migration;
pub mod proxy;
use std::{path::PathBuf, str::FromStr};

use sqlx::{
    self,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

use crate::db::migration::Migration;

pub type DatabaseDialect = sqlx::Sqlite;
pub type DatabasePool = sqlx::Pool<DatabaseDialect>;

pub struct Database {
    db_dir: PathBuf,
    db_name: String,
    pool: DatabasePool,
}

impl Database {
    pub const DEFAULT_DB_NAME: &'static str = "app.db";

    pub async fn new(
        password: &str,
        db_dir: PathBuf,
        db_name: Option<&str>,
    ) -> Result<Self, String> {
        let db_name = db_name.unwrap_or(Self::DEFAULT_DB_NAME);
        let db_url = db_dir.join(db_name);

        let connect_options =
            SqliteConnectOptions::from_str(&db_url.to_str().ok_or("Invalid db path")?)
                .map_err(|err| err.to_string())?
                .pragma("key", password.to_string())
                .pragma("journal_mode", "WAL".to_string())
                .pragma("synchronous", "NORMAL".to_string())
                .pragma("busy_timeout", "5000".to_string())
                .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .connect_with(connect_options)
            .await
            .map_err(|err| err.to_string())?;

        Migration::setup_migration_table(&pool)
            .await
            .map_err(|err| err.to_string())?;

        Ok(Self {
            pool: pool,
            db_dir: db_dir,
            db_name: String::from(db_name),
        })
    }

    pub fn get_pool(&self) -> &DatabasePool {
        return &self.pool;
    }

    pub async fn close_pool(&self) {
        self.pool.close().await
    }

    pub fn get_db_name(&self) -> String {
        self.db_name.clone()
    }

    pub async fn is_ready(&self) -> bool {
        let row: Option<i32> = sqlx::query_scalar(
            format!(
                "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='{}';",
                Migration::MIGRATION_TABLE_NAME
            )
            .as_str(),
        )
        .fetch_one(self.get_pool())
        .await
        .ok();

        if let Some(count) = row {
            return count == 1;
        }
        false
    }

    pub async fn reset(&self) -> Result<(), String> {
        self.close_pool().await;
        Self::purge_data(&self.db_dir)?;
        Ok(())
    }

    pub fn purge_data(db_dir: &PathBuf) -> Result<(), String> {
        let entries =
            std::fs::read_dir(db_dir).map_err(|e| format!("Failed to read db directory: {}", e))?;
        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if let Some(fname) = path.file_name().and_then(|n| n.to_str()) {
                if fname.starts_with(Database::DEFAULT_DB_NAME) {
                    std::fs::remove_file(&path)
                        .map_err(|e| format!("Failed to delete {}: {}", fname, e))?;
                }
            }
        }
        Ok(())
    }
}
