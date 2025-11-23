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
        "INTEGER" => row
            .try_get::<i64, _>(index)
            .map(Value::from)
            .unwrap_or(Value::Null),
        "REAL" => row
            .try_get::<f64, _>(index)
            .map(Value::from)
            .unwrap_or(Value::Null),
        "TEXT" => row
            .try_get::<String, _>(index)
            .map(Value::String)
            .unwrap_or(Value::Null),
        "BLOB" => row
            .try_get::<Vec<u8>, _>(index)
            .map(|bytes| Value::String(general_purpose::STANDARD.encode(&bytes)))
            .unwrap_or(Value::Null),
        _ => row
            .try_get::<String, _>(index)
            .map(Value::String)
            .unwrap_or(Value::Null),
    }
}

#[tauri::command]
pub async fn execute_single_sql(
    app_state: tauri::State<'_, AppState>,
    query: SQLQuery,
) -> Result<Vec<SQLRow>, String> {
    let db = app_state.db.clone();

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
    let db = app_state.db.clone();

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
