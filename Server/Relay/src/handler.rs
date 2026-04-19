use crate::{state::ServerState, thorium};
use prost::Message;
use quinn::Connection;

pub async fn handle_connection(connection: Connection, state: ServerState) {
    let remote_addr = connection.remote_address();
    
    while let Ok((mut send_stream, mut recv_stream)) = connection.accept_bi().await {
        let state = state.clone();
        
        tokio::spawn(async move {
            let mut buf = vec![0; 2048];
            if let Ok(Some(n)) = recv_stream.read(&mut buf).await {
                if let Ok(packet) = thorium::Packet::decode(&buf[..n]) {
                    if let Some(payload) = packet.payload {
                        match payload {
                            thorium::packet::Payload::Envelope(env) => {
                                if env.destination_id == "server" {
                                    let mut map = state.peers.lock().await;
                                    map.insert(env.sender_id.clone(), remote_addr);
                                    println!("registered peer: {} at {}", env.sender_id, remote_addr);
                                }
                            }
                            thorium::packet::Payload::PeerRequest(req) => {
                                println!("peer request for {}", req.target_id);
                                let map = state.peers.lock().await;
                                let ip_str = if let Some(addr) = map.get(&req.target_id) {
                                    addr.to_string()
                                } else {
                                    "".to_string()
                                };

                                let response = thorium::Packet {
                                    payload: Some(thorium::packet::Payload::PeerResponse(thorium::PeerInfo {
                                        target_id: req.target_id,
                                        ip_address: ip_str,
                                    })),
                                };
                                
                                let mut resp_buf = Vec::new();
                                response.encode(&mut resp_buf).unwrap();
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
