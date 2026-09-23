use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct ServiceStatus {
    pub up: bool,
}

pub type SharedState = Arc<Mutex<HashMap<String, ServiceStatus>>>;

pub fn init() -> SharedState {
    Arc::new(Mutex::new(HashMap::new()))
}