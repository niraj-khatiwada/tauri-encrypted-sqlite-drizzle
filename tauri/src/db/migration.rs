use sqlparser::dialect::SQLiteDialect;
use sqlparser::parser::Parser;
use std::fs;
use std::path::{Path, PathBuf};

use crate::db::DatabasePool;

pub struct Migration {
    pool: DatabasePool,
    migrations_dir: PathBuf,
}

impl Migration {
    pub const MIGRATION_TABLE_NAME: &'static str = "__migration__";

    pub fn new(pool: DatabasePool, migrations_dir: PathBuf) -> Self {
        Self {
            pool,
            migrations_dir,
        }
    }

    pub async fn run(&self) -> Result<(), String> {
        println!("[migration] Running SQL migrations.");
        Self::setup_migration_table(&self.pool).await?;

        let migration_files = self.get_migration_files()?;
        let mut migrations_count = 0;

        for file in migration_files {
            let file_name = file.clone();
            let sql = fs::read_to_string(format!(
                "{}/{}",
                self.migrations_dir.to_string_lossy().to_string(),
                file
            ))
            .map_err(|e| format!("Failed to read migration {}: {}", file, e))?;

            if self.is_migration_applied(&file_name).await? {
                continue;
            }

            migrations_count += 1;
            println!("[migration] Applying migration: {}", file_name);
            if let Err(err) = self.apply_migration(&file_name, &sql).await {
                println!(
                    "[migration] Migration failed: {}\nError: {}",
                    file_name, err
                );
                return Err(err);
            }

            println!("[migration] Migration applied: {}", file_name);
        }

        println!(
            "[migration] Migration completed. {} new migrations applied.",
            migrations_count
        );

        Ok(())
    }

    pub async fn setup_migration_table(pool: &DatabasePool) -> Result<(), String> {
        sqlx::query(
            format!(
                "CREATE TABLE IF NOT EXISTS {}(id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL UNIQUE, applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP);",
                Self::MIGRATION_TABLE_NAME
            )
            .as_str(),
        )
        .execute(pool)
        .await
        .map_err(|err| err.to_string())?;
        Ok(())
    }

    fn get_migration_files(&self) -> Result<Vec<String>, String> {
        let path = Path::new(&self.migrations_dir);

        if !path.exists() {
            return Err(format!(
                "Migration folder not found: {}",
                self.migrations_dir.to_string_lossy().to_string()
            ));
        }

        let mut files: Vec<String> = fs::read_dir(path)
            .map_err(|e| e.to_string())?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()?.to_str()? == "sql" {
                    Some(path.file_name()?.to_string_lossy().to_string())
                } else {
                    None
                }
            })
            .collect();

        files.sort();
        Ok(files)
    }

    async fn is_migration_applied(&self, name: &str) -> Result<bool, String> {
        let res: Option<(i64,)> =
            sqlx::query_as("SELECT id FROM __migrations__ WHERE name = ? LIMIT 1;")
                .bind(name)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

        Ok(res.is_some())
    }

    async fn apply_migration(&self, name: &str, sql: &str) -> Result<(), String> {
        let dialect = SQLiteDialect {};
        let statements = Parser::parse_sql(&dialect, sql).map_err(|e| e.to_string())?;

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        for statement in statements {
            let sql_str = statement.to_string();
            sqlx::query(&sql_str)
                .execute(&mut *tx)
                .await
                .map_err(|e| format!("{}: {}", name, e))?;
        }

        sqlx::query("INSERT INTO __migrations__ (name) VALUES (?)")
            .bind(name)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }
}
