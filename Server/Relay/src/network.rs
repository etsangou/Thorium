use crate::config::Config;
use quinn::{ServerConfig, TransportConfig};
use std::{error::Error, sync::Arc, time::Duration};

pub fn configure_server(app_config: &Config) -> Result<ServerConfig, Box<dyn Error>> {
    let cert = rcgen::generate_simple_self_signed(vec![app_config.server.cert_hostname.clone()])?;
    let cert_der = rustls::Certificate(cert.serialize_der()?);
    let priv_key = rustls::PrivateKey(cert.serialize_private_key_der());
    
    let crypto = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(vec![cert_der], priv_key)?;

    let mut transport_config = TransportConfig::default();
    transport_config.max_idle_timeout(Some(Duration::from_secs(app_config.network.idle_timeout_secs).try_into().unwrap()));

    let mut server_config = ServerConfig::with_crypto(Arc::new(crypto));
    server_config.transport_config(Arc::new(transport_config));

    Ok(server_config)
}
