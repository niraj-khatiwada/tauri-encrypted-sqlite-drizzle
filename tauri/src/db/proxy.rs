use crate::domain::AppState;
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{sqlite::SqliteRow, Column, Row, TypeInfo};

#[derive(Debug, Serialize, Deserialize)]
pub struct SQLQuery {
    pub sql: String,
    pub params: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SQLRow {
    pub columns: Vec<String>,
    pub rows: Vec<serde_json::Value>,
}

#[tauri::command]
pub async fn execute_single_sql(
    app_state: tauri::State<'_, AppState>,
    query: SQLQuery,
) -> Result<Vec<SQLRow>, String> {
    let db = app_state.db.clone();

    let sql = query.sql.as_str();
    let params = query.params;

    let mut query = sqlx::query(sql);

    for p in params {
        match p {
            Value::String(s) => query = query.bind(s),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    query = query.bind(i);
                } else if let Some(f) = n.as_f64() {
                    query = query.bind(f);
                }
            }
            Value::Bool(b) => query = query.bind(b),
            _ => query = query.bind(None::<String>),
        }
    }

    let rows = query.fetch_all(&db.pool).await.map_err(|e| e.to_string())?;
    let rows = rows
        .iter()
        .map(|row| {
            let columns: Vec<String> = row.columns().iter().map(|c| c.name().to_string()).collect();
            let rows = (0..row.len())
                .map(|i| match row.try_get_raw(i) {
                    Ok(_) => sqlx_value_to_json(row, i),
                    Err(_) => Value::Null,
                })
                .collect();

            SQLRow { columns, rows }
        })
        .collect();

    Ok(rows)
}

fn sqlx_value_to_json(row: &SqliteRow, index: usize) -> Value {
    let column = row.column(index);
    let type_name = column.type_info().name();

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
