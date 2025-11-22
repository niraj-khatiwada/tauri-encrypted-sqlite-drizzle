use std::fs;
use std::path::PathBuf;
use tauri::Manager;
use tauri_plugin_fs::FsExt;

/// Create required db directory if not exists.
pub fn setup_db_dir(app: &mut tauri::App) -> Result<PathBuf, tauri::Error> {
    let scope = app.fs_scope();
    let app_data_directory = app.path().app_data_dir()?;
    scope.allow_directory(&app_data_directory, true)?;

    fs::create_dir_all(format!("{}/db", &app_data_directory.display().to_string()))?;

    Ok(app_data_directory)
}
