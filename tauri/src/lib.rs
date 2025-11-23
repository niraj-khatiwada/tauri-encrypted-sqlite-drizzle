pub mod commands;
pub mod db;
pub mod domain;
pub mod fs;
use domain::AppState;
#[cfg(debug_assertions)]
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{path::BaseDirectory, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let db_path = fs::setup_db_dir(app).unwrap_or_else(|err| panic!("{}", err));

            let password = "super_secret_password"; // TODO: Store this somewhere safe or ask the user

            let db = tauri::async_runtime::block_on(db::Database::new(password, &db_path, None))
                .unwrap_or_else(|err| panic!("{}", err));

            #[cfg(not(debug_assertions))]
            let migration_dir = app.path().resolve("migrations", BaseDirectory::Resource)?;
            #[cfg(debug_assertions)]
            let migration_dir = PathBuf::new().join("migrations");

            let migration = db::migration::Migration::new(db.get_pool().clone(), migration_dir);
            tauri::async_runtime::block_on(migration.run()).unwrap_or_else(|err| panic!("{}", err));

            let app_state = AppState { db: Arc::new(db) };
            app.manage(app_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::is_db_ready,
            db::proxy::execute_single_sql,
            db::proxy::execute_batch_sql,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, _event| {});
}
