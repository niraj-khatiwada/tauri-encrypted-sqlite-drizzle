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
            let db_path = match fs::setup_db_dir(app) {
                Ok(mut path) => {
                    path.push("db.sqlite");
                    path
                }
                Err(err) => panic!("{}", err),
            };

            println!(">> {:?}", db_path);

            let db = match tauri::async_runtime::block_on(db::Database::new(
                &db_path,
                "super_secret_password",
            )) {
                Ok(ok) => ok,
                Err(err) => panic!("{}", err),
            };

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
