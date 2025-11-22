use std::{path::PathBuf, str::FromStr};

use sqlx::{
    self,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

pub type DatabaseDialect = sqlx::Sqlite;
pub type DatabasePool = sqlx::Pool<DatabaseDialect>;

pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<DatabaseDialect> {
    pub async fn new(db_path: &PathBuf, password: &str) -> Result<Self, String> {
        let db_url = match db_path.to_str() {
            Some(url) => url,
            None => return Err(String::from("Invalid db url.")),
        };
        let connect_options = SqliteConnectOptions::from_str(db_url)
            .map_err(|err| err.to_string())?
            .pragma("key", String::from(password));

        let pool = SqlitePoolOptions::new()
            .connect_with(connect_options)
            .await
            .map_err(|err| err.to_string())?;

        Ok(Self(pool))
    }

    pub fn get_pool(&self) -> &sqlx::Pool<DatabaseDialect> {
        return &self.0;
    }
}
