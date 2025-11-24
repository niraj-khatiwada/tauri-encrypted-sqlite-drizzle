use crate::domain::AppState;
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{sqlite::SqliteRow, Column, Row, Sqlite, Transaction, TypeInfo};

#[derive(Debug, Serialize, Deserialize)]
pub struct SQLQuery {
    pub sql: String,
    pub params: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SQLRow {
    pub columns: Vec<String>,
    pub rows: Vec<Value>,
}

fn bind_params<'a>(
    mut query: sqlx::query::Query<'a, Sqlite, sqlx::sqlite::SqliteArguments<'a>>,
    params: &'a [Value],
) -> sqlx::query::Query<'a, Sqlite, sqlx::sqlite::SqliteArguments<'a>> {
    for p in params {
        match p {
            Value::String(s) => query = query.bind(s),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    query = query.bind(i);
                } else if let Some(f) = n.as_f64() {
                    query = query.bind(f);
                } else {
                    query = query.bind(None::<String>);
                }
            }
            Value::Bool(b) => query = query.bind(*b),
            Value::Null => query = query.bind(None::<String>),
            _ => query = query.bind(None::<String>),
        }
    }

    query
}

fn row_to_sql_row(row: &SqliteRow) -> SQLRow {
    let columns: Vec<String> = row.columns().iter().map(|c| c.name().to_string()).collect();
    let values: Vec<Value> = (0..row.len()).map(|i| sqlx_value_to_json(row, i)).collect();

    SQLRow {
        columns,
        rows: values,
    }
}

fn sqlx_value_to_json(row: &SqliteRow, index: usize) -> Value {
    let col = row.column(index);
    let type_name = col.type_info().name();

    match type_name {
        "INTEGER" => match row.try_get::<Option<i64>, _>(index) {
            Ok(Some(i)) => Value::from(i),
            Ok(None) => Value::Null,
            Err(_) => Value::Null,
        },
        "REAL" => match row.try_get::<Option<f64>, _>(index) {
            Ok(Some(f)) => Value::from(f),
            Ok(None) => Value::Null,
            Err(_) => Value::Null,
        },
        "TEXT" => match row.try_get::<Option<String>, _>(index) {
            Ok(Some(s)) => Value::String(s),
            Ok(None) => Value::Null,
            Err(_) => Value::Null,
        },
        "BLOB" => match row.try_get::<Option<Vec<u8>>, _>(index) {
            Ok(Some(bytes)) => Value::String(general_purpose::STANDARD.encode(&bytes)),
            Ok(None) => Value::Null,
            Err(_) => Value::Null,
        },
        _ => match row.try_get::<Option<String>, _>(index) {
            Ok(Some(s)) => Value::String(s),
            Ok(None) => Value::Null,
            Err(_) => Value::Null,
        },
    }
}

fn log_sql_proxy(single: Option<&SQLQuery>, batch: Option<&[SQLQuery]>) {
    if let Some(query) = single {
        println!(
            "[proxy] Single SQL: {}; PARAMS: {:?}",
            query.sql,
            query
                .params
                .iter()
                .map(|p| serde_json::to_string_pretty(&p.to_string()))
                .collect::<Vec<_>>()
        );
    }

    if let Some(queries) = batch {
        println!(
            "[proxy] Batch SQL: {} queries\n{}",
            queries.len(),
            queries
                .iter()
                .map(|q| format!(
                    "SQL: {}; PARAMS: {:?}",
                    q.sql,
                    q.params
                        .iter()
                        .map(|p| serde_json::to_string_pretty(&p.to_string()))
                        .collect::<Vec<_>>()
                ))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}

#[tauri::command]
pub async fn execute_single_sql(
    app_state: tauri::State<'_, AppState>,
    query: SQLQuery,
) -> Result<Vec<SQLRow>, String> {
    #[cfg(debug_assertions)]
    log_sql_proxy(Some(&query), None);

    let sql_upper = query.sql.trim().to_uppercase();
    if sql_upper.starts_with("BEGIN")
        || sql_upper.starts_with("COMMIT")
        || sql_upper.starts_with("ROLLBACK")
    {
        return Err(
            "BEGIN/COMMIT/ROLLBACK not allowed in single SQL execution. Use batch execution instead.".into(),
        );
    }

    let db_ref = app_state.db.read().await;
    let db = db_ref
        .as_ref()
        .ok_or("Database not initialized".to_string())?;

    let mut q = sqlx::query(query.sql.as_str());
    q = bind_params(q, &query.params);

    let rows = q.fetch_all(&db.pool).await.map_err(|e| e.to_string())?;

    Ok(rows.iter().map(row_to_sql_row).collect())
}

#[tauri::command]
pub async fn execute_batch_sql(
    app_state: tauri::State<'_, AppState>,
    queries: Vec<SQLQuery>,
) -> Result<Vec<Vec<SQLRow>>, String> {
    #[cfg(debug_assertions)]
    log_sql_proxy(None, Some(&queries));

    let db_ref = app_state.db.read().await;
    let db = db_ref
        .as_ref()
        .ok_or("Database not initialized".to_string())?;

    let mut tx: Transaction<'_, Sqlite> = db.pool.begin().await.map_err(|e| e.to_string())?;

    let mut results: Vec<Vec<SQLRow>> = vec![];

    for query in queries {
        let mut q = sqlx::query(query.sql.as_str());
        q = bind_params(q, &query.params);

        let rows = q
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| format!("Error executing '{}': {}", query.sql, e))?;

        let converted: Vec<SQLRow> = rows.iter().map(row_to_sql_row).collect();
        results.push(converted);
    }

    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(results)
}
