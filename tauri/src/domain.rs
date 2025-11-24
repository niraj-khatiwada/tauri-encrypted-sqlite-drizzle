use tokio::sync::RwLock;

use crate::db::Database;
use std::{path::PathBuf, sync::Arc};

pub struct AppState {
    pub db: Arc<RwLock<Option<Database>>>,
    pub db_dir: PathBuf,
    pub migration_dir: PathBuf,
}
