use async_lock::RwLock;
use async_session::Session;
use std::{collections::HashMap, sync::Arc};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref MEMORY_SESSION: Arc<RwLock<HashMap<String, Session>>> =
        Arc::new(RwLock::new(HashMap::new()));
    pub static ref USER_MAP: Arc<RwLock<HashMap<String, String>>> =
        Arc::new(RwLock::new(HashMap::new()));
}
