use chacha20poly1305::{aead::{Aead, KeyInit, OsRng, AeadCore}, ChaCha20Poly1305, Key};
use prost::Message;
use quinn::{ClientConfig, Endpoint, TransportConfig};
use std::{error::Error, io::{self, Write}, sync::Arc, time::Duration};

pub mod thorium {
    include!(concat!(env!("OUT_DIR"), "/thorium.rs"));
}

struct SkipServerVerification;
impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(&self, _e: &rustls::Certificate, _i: &[rustls::Certificate], _s: &rustls::ServerName, _sc: &mut dyn Iterator<Item = &[u8]>, _oc: &[u8], _n: std::time::SystemTime) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let crypto = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(Arc::new(SkipServerVerification))
        .with_no_client_auth();

    let mut transport_config = TransportConfig::default();
    transport_config.max_idle_timeout(Some(Duration::from_secs(300).try_into().unwrap()));
    transport_config.keep_alive_interval(Some(Duration::from_secs(5)));

    let mut client_config = ClientConfig::new(Arc::new(crypto));
    client_config.transport_config(Arc::new(transport_config));

    let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
    endpoint.set_default_client_config(client_config);

    let connection = endpoint.connect("127.0.0.1:4433".parse()?, "localhost")?.await?;
    println!("connected. enter your ID:");
    let mut my_id = String::new();
    io::stdin().read_line(&mut my_id)?;
    let my_id = my_id.trim().to_string();

    let (mut auth_send, _) = connection.open_bi().await?;
    let auth_packet = thorium::Packet {
        payload: Some(thorium::packet::Payload::Envelope(thorium::QuicEnvelope {
            message_id: "auth".into(),
            r#type: 0,
            timestamp: 0,
            sender_id: my_id.clone(),
            destination_id: "server".into(),
            encrypted_payload: vec![],
            channel_id: "".into(),
        })),
    };
    let mut auth_buf = Vec::new();
    auth_packet.encode(&mut auth_buf)?;
    auth_send.write_all(&auth_buf).await?;
    auth_send.finish().await?;

    let shared_key = Key::from_slice(b"thorium_super_secret_key_32bytes");
    let cipher = ChaCha20Poly1305::new(shared_key);

    let listen_conn = connection.clone();
    let listen_cipher = ChaCha20Poly1305::new(shared_key);
    tokio::spawn(async move {
        while let Ok((_, mut recv)) = listen_conn.accept_bi().await {
            let mut buf = vec![0; 2048];
            if let Ok(Some(n)) = recv.read(&mut buf).await {
                if let Ok(packet) = thorium::Packet::decode(&buf[..n]) {
                    if let Some(thorium::packet::Payload::Envelope(env)) = packet.payload {
                        if env.encrypted_payload.len() > 12 {
                            let nonce = chacha20poly1305::Nonce::from_slice(&env.encrypted_payload[..12]);
                            let ciphertext = &env.encrypted_payload[12..];
                            if let Ok(decrypted) = listen_cipher.decrypt(nonce, ciphertext) {
                                let prefix = if env.channel_id.is_empty() {
                                    format!("[FROM {}]", env.sender_id)
                                } else {
                                    format!("[{} - {}]", env.sender_id, env.channel_id)
                                };
                                println!("\n{}: {}", prefix, String::from_utf8_lossy(&decrypted));
                                print!("> "); io::stdout().flush().unwrap();
                            }
                        }
                    }
                }
            }
        }
    });

    loop {
        println!("\n--- THORIUM MENU ---");
        println!("1. Refresh client list");
        println!("2. Start direct chat");
        println!("3. Join space salon");
        println!("4. Quit");
        print!("> "); io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                let (mut send, mut recv) = connection.open_bi().await?;
                let packet = thorium::Packet {
                    payload: Some(thorium::packet::Payload::ListRequest(thorium::ClientListRequest {})),
                };
                let mut buf = Vec::new();
                packet.encode(&mut buf)?;
                send.write_all(&buf).await?;
                send.finish().await?;

                let mut resp_buf = vec![0; 2048];
                if let Ok(Some(n)) = recv.read(&mut resp_buf).await {
                    if let Ok(res) = thorium::Packet::decode(&resp_buf[..n]) {
                        if let Some(thorium::packet::Payload::ListResponse(list)) = res.payload {
                            println!("Online clients: {:?}", list.client_ids);
                        }
                    }
                }
            }
            "2" => {
                println!("target ID:");
                let mut target_id = String::new();
                io::stdin().read_line(&mut target_id)?;
                let target_id = target_id.trim().to_string();

                println!("chatting directly with {}. type 'quit' to return.", target_id);
                chat_loop(&connection, &cipher, &my_id, target_id, "".to_string(), false).await?;
            }
            "3" => {
                println!("space ID:");
                let mut space_id = String::new();
                io::stdin().read_line(&mut space_id)?;
                let space_id = space_id.trim().to_string();

                println!("salon name:");
                let mut salon_id = String::new();
                io::stdin().read_line(&mut salon_id)?;
                let salon_id = salon_id.trim().to_string();

                println!("broadcasting to {} in {}. type 'quit' to return.", salon_id, space_id);
                chat_loop(&connection, &cipher, &my_id, space_id, salon_id, true).await?;
            }
            "4" => break,
            _ => println!("invalid choice"),
        }
    }
    Ok(())
}

async fn chat_loop(
    connection: &quinn::Connection,
    cipher: &ChaCha20Poly1305,
    my_id: &str,
    target_id: String,
    channel_id: String,
    is_group: bool,
) -> Result<(), Box<dyn Error>> {
    loop {
        print!("chat > "); io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let text = input.trim();
        if text == "quit" { break; }
        if text.is_empty() { continue; }

        let formatted_text = if is_group {
            format!("{}: {}", my_id, text)
        } else {
            text.to_string()
        };

        let (mut send, _) = connection.open_bi().await?;
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&nonce, formatted_text.as_bytes()).unwrap();
        let mut payload = nonce.to_vec();
        payload.extend_from_slice(&ciphertext);

        let packet = thorium::Packet {
            payload: Some(thorium::packet::Payload::Envelope(thorium::QuicEnvelope {
                message_id: "msg".into(),
                r#type: 1,
                timestamp: 0,
                sender_id: my_id.to_string(),
                destination_id: target_id.clone(),
                encrypted_payload: payload,
                channel_id: channel_id.clone(),
            })),
        };

        let mut buf = Vec::new();
        packet.encode(&mut buf)?;
        send.write_all(&buf).await?;
        send.finish().await?;
    }
    Ok(())
}
