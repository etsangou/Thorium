use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ServerState {
    pub peers: Arc<Mutex<HashMap<String, SocketAddr>>>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            peers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
