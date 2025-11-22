pub mod db;
pub mod domain;
pub mod fs;
use domain::AppState;
use std::sync::Arc;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let db_path = fs::setup_db_dir(app).unwrap_or_else(|err| panic!("{}", err));

            let password = "super_secret_password"; // TODO: Store this somewhere safe or ask the user

            let db = tauri::async_runtime::block_on(db::Database::new(password, &db_path, None))
                .unwrap_or_else(|err| panic!("{}", err));

            println!(
                "is connected {}",
                tauri::async_runtime::block_on(db.is_connected())
            );

            let app_state = AppState {
                db: Arc::new(db.get_pool().clone()),
            };
            app.manage(app_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| {});
}
