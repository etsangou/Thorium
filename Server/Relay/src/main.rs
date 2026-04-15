use prost::Message;
use quinn::{Endpoint, ServerConfig, Connection, TransportConfig};
use std::{error::Error, net::SocketAddr, sync::Arc, collections::HashMap, time::Duration};
use tokio::sync::Mutex;

pub mod thorium {
    include!(concat!(env!("OUT_DIR"), "/thorium.rs"));
}

type ClientMap = Arc<Mutex<HashMap<String, Connection>>>;

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

    let mut transport_config = TransportConfig::default();
    transport_config.max_idle_timeout(Some(Duration::from_secs(300).try_into().unwrap()));

    let mut server_config = ServerConfig::with_crypto(Arc::new(crypto));
    server_config.transport_config(Arc::new(transport_config));

    let endpoint = Endpoint::server(server_config, addr)?;
    let clients: ClientMap = Arc::new(Mutex::new(HashMap::new()));

    println!("Thorium relay listening on {}", endpoint.local_addr()?);

    while let Some(conn) = endpoint.accept().await {
        let clients = Arc::clone(&clients);
        tokio::spawn(async move {
            match conn.await {
                Ok(connection) => {
                    println!("new connection from {}", connection.remote_address());
                    
                    while let Ok((mut send_stream, mut recv_stream)) = connection.accept_bi().await {
                        let clients = Arc::clone(&clients);
                        let connection = connection.clone();
                        
                        tokio::spawn(async move {
                            let mut buf = vec![0; 2048];
                            if let Ok(Some(n)) = recv_stream.read(&mut buf).await {
                                if let Ok(packet) = thorium::Packet::decode(&buf[..n]) {
                                    if let Some(payload) = packet.payload {
                                        match payload {
                                            thorium::packet::Payload::ListRequest(_) => {
                                                let map = clients.lock().await;
                                                let ids: Vec<String> = map.keys().cloned().collect();
                                                
                                                let response = thorium::Packet {
                                                    payload: Some(thorium::packet::Payload::ListResponse(thorium::ClientListResponse {
                                                        client_ids: ids,
                                                    })),
                                                };
                                                let mut resp_buf = Vec::new();
                                                response.encode(&mut resp_buf).unwrap();
                                                let _ = send_stream.write_all(&resp_buf).await;
                                                let _ = send_stream.finish().await;
                                            }
                                            thorium::packet::Payload::Envelope(env) => {
                                                {
                                                    let mut map = clients.lock().await;
                                                    map.insert(env.sender_id.clone(), connection.clone());
                                                }

                                                let dest_id = env.destination_id.clone();
                                                if dest_id == "server" {
                                                    println!("registered client: {}", env.sender_id);
                                                    return;
                                                }

                                                let garbage = String::from_utf8_lossy(&env.encrypted_payload);
                                                println!("routing message to {} | payload: {}", dest_id, garbage);

                                                let mut routed = false;
                                                let target_conn = {
                                                    let map = clients.lock().await;
                                                    map.get(&dest_id).cloned()
                                                };

                                                if let Some(conn) = target_conn {
                                                    if let Ok((mut t_send, _)) = conn.open_bi().await {
                                                        let mut forward_buf = Vec::new();
                                                        let forward_packet = thorium::Packet {
                                                            payload: Some(thorium::packet::Payload::Envelope(env.clone())),
                                                        };
                                                        forward_packet.encode(&mut forward_buf).unwrap();
                                                        if t_send.write_all(&forward_buf).await.is_ok() {
                                                            let _ = t_send.finish().await;
                                                            routed = true;
                                                        }
                                                    }
                                                }

                                                let ack = thorium::ServerAck {
                                                    message_id: env.message_id,
                                                    succes: routed,
                                                    error_info: if routed { "delivered".into() } else { "offline".into() },
                                                };
                                                let mut resp_buf = Vec::new();
                                                thorium::Packet { payload: Some(thorium::packet::Payload::Ack(ack)) }.encode(&mut resp_buf).unwrap();
                                                let _ = send_stream.write_all(&resp_buf).await;
                                                let _ = send_stream.finish().await;
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        });
                    }
                }
                Err(e) => println!("connection error: {}", e),
            }
        });
    }
    Ok(())
}
