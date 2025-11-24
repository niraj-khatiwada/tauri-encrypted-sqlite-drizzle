use crate::{db, domain::AppState};

#[tauri::command]
pub async fn is_db_ready(app_state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let db_lock = app_state.db.read().await;
    let db = match db_lock.as_ref() {
        Some(_db) => _db,
        _ => return Ok(false),
    };
    Ok(db.is_ready().await)
}

#[tauri::command]
pub async fn init_db(
    app_state: tauri::State<'_, AppState>,
    encryption_key: &str,
) -> Result<(), String> {
    {
        let db_lock = app_state.db.read().await;
        if db_lock.is_some() {
            return Err(String::from("Database is already initialized."));
        }
    }

    let db = db::Database::new(encryption_key, app_state.db_dir.clone(), None).await?;

    let migration =
        db::migration::Migration::new(db.get_pool().clone(), app_state.migration_dir.clone());
    migration.run().await?;

    let mut db_state = app_state.db.write().await;
    *db_state = Some(db);

    Ok(())
}
