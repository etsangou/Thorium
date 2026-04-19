use crate::{handler, state::ServerState};
use quinn::Endpoint;

pub async fn run(endpoint: Endpoint, state: ServerState) {
    while let Some(conn) = endpoint.accept().await {
        let state_clone = state.clone();
        tokio::spawn(async move {
            if let Ok(connection) = conn.await {
                let remote_addr = connection.remote_address();
                println!("incoming connection from {}", remote_addr);
                handler::handle_connection(connection, state_clone).await;
            } else {
                println!("connection failed");
            }
        });
    }
}
