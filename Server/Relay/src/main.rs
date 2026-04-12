use prost::Message;
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

                    while let Ok((mut send_stream, mut recv_stream)) = connection.accept_bi().await {
                        let mut buf = vec![0; 1024];
                        if let Ok(Some(n)) = recv_stream.read(&mut buf).await {
                            if let Ok(packet) = thorium::Packet::decode(&buf[..n]) {
                                if let Some(thorium::packet::Payload::Envelope(env)) = packet.payload {
                                    let payload_text = String::from_utf8_lossy(&env.encrypted_payload);
                                    println!("Received for {}: {}", env.destination_id, payload_text);

                                    let ack = thorium::ServerAck {
                                        message_id: env.message_id,
                                        succes: true,
                                        error_info: "Message received".to_string(),
                                    };

                                    let response_packet = thorium::Packet {
                                        payload: Some(thorium::packet::Payload::Ack(ack)),
                                    };

                                    let mut resp_buf = Vec::new();
                                    response_packet.encode(&mut resp_buf).unwrap();
                                    let _ = send_stream.write_all(&resp_buf).await;
                                }
                            }
                        }
                    }
                }
                Err(e) => println!("connection error: {}", e),
            }
        });
    }

    Ok(())
}
