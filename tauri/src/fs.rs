use std::fs;
use std::path::PathBuf;
use tauri::Manager;
use tauri_plugin_fs::FsExt;

/// Create required db directory if it does not exist.
pub fn setup_db_dir(app: &mut tauri::App) -> Result<PathBuf, tauri::Error> {
    let scope = app.fs_scope();
    let app_data_directory = app.path().app_data_dir()?;
    scope.allow_directory(&app_data_directory, true)?;

    let db_dir = &app_data_directory.display().to_string();
    fs::create_dir_all(format!("{}", db_dir))?;

    Ok(PathBuf::from(db_dir))
}
