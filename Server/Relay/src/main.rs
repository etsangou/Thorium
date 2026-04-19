mod config;
mod handler;
mod network;
mod server;
mod state;

pub mod thorium {
    include!(concat!(env!("OUT_DIR"), "/thorium.rs"));
}

use std::{error::Error, net::SocketAddr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app_config = config::load();
    let addr_str = format!("{}:{}", app_config.server.ip, app_config.server.port);
    let addr: SocketAddr = addr_str.parse()?;
    
    let state = state::ServerState::new();
    let server_config = network::configure_server(&app_config)?;
    
    let endpoint = quinn::Endpoint::server(server_config, addr)?;
    println!("Signaling server listening on {}", endpoint.local_addr()?);
    
    server::run(endpoint, state).await;
    
    Ok(())
}
