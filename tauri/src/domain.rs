use crate::db::DatabasePool;
use std::sync::Arc;

pub struct AppState {
    pub db: Arc<DatabasePool>,
}
