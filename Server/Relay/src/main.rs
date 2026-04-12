use quinn::{Endpoint, ServerConfig};
use std::{error::Error, net::SocketAddr, sync::Arc};

pub mod thorium {
	include!(concat!(env!("OUT_DIR"), "/thorium.rs"));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = "0.0.0.0:4433".parse()?;

    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
    let cert_der = rustls::Certificate(cert.serialize_der()?);
    let priv_key = rustls::PrivateKey(cert.serialize_private_key_der());
    let crypto = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(vec![cert_der], priv_key)?;

    let server_config = ServerConfig::with_crypto(Arc::new(crypto));

    let endpoint = Endpoint::server(server_config, addr)?;
    println!("Thorium server listening on {}", endpoint.local_addr()?);

    while let Some(conn) = endpoint.accept().await {
        tokio::spawn(async move {
            println!("connection with a client...");

            match conn.await {
                Ok(connection) => {
                    println!("client connected, ID: {}", connection.remote_address());
                }
                Err(e) => println!("connexion error: {}", e),
            }
        });
    }

    Ok(())
}
