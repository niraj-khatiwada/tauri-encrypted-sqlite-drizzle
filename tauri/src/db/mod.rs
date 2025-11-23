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
    db_name: String,
    pool: DatabasePool,
}

impl Database {
    #[cfg(debug_assertions)]
    const DEFAULT_DB_NAME: &'static str = "app_dev.db";
    #[cfg(not(debug_assertions))]
    const DEFAULT_DB_NAME: &'static str = "app.db";

    pub async fn new(
        password: &str,
        db_dir: &PathBuf,
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
            db_name: String::from(db_name),
        })
    }

    pub fn get_pool(&self) -> &DatabasePool {
        return &self.pool;
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
}
