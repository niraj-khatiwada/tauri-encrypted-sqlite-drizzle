use crate::domain::AppState;

#[tauri::command]
pub async fn is_db_ready(app_state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let db = app_state.db.clone();
    Ok(db.is_ready().await)
}
