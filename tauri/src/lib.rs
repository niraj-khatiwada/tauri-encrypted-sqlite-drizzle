pub mod commands;
pub mod db;
pub mod domain;
pub mod fs;
use domain::AppState;
#[cfg(debug_assertions)]
use std::path::PathBuf;
use std::sync::Arc;
#[cfg(not(debug_assertions))]
use tauri::path::BaseDirectory;
use tauri::Manager;
use tokio::sync::RwLock;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let db_dir = fs::setup_db_dir(app).unwrap_or_else(|err| panic!("{}", err));

            #[cfg(not(debug_assertions))]
            let migration_dir = app.path().resolve("migrations", BaseDirectory::Resource)?;
            #[cfg(debug_assertions)]
            let migration_dir = PathBuf::new().join("migrations");

            let app_state = AppState {
                db: Arc::new(RwLock::new(None)),
                db_dir: db_dir,
                migration_dir: migration_dir,
            };
            app.manage(app_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::is_db_ready,
            commands::init_db,
            db::proxy::execute_single_sql,
            db::proxy::execute_batch_sql,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, _event| {});
}
