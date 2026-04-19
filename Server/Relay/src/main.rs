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
    let addr: SocketAddr = "0.0.0.0:4433".parse()?;
    
    let state = state::ServerState::new();
    let server_config = network::configure_server()?;
    
    let endpoint = quinn::Endpoint::server(server_config, addr)?;
    println!("Signaling server listening on {}", endpoint.local_addr()?);
    
    server::run(endpoint, state).await;
    
    Ok(())
}
