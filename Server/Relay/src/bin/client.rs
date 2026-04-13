use chacha20poly1305::{aead::{Aead, KeyInit, OsRng, AeadCore}, ChaCha20Poly1305, Key};
use prost::Message;
use quinn::{ClientConfig, Endpoint, Connection};
use std::{error::Error, io::{self, Write}, sync::Arc};

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
    let crypto = rustls::ClientConfig::builder().with_safe_defaults().with_custom_certificate_verifier(Arc::new(SkipServerVerification)).with_no_client_auth();
    let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
    endpoint.set_default_client_config(ClientConfig::new(Arc::new(crypto)));

    let connection = endpoint.connect("127.0.0.1:4433".parse()?, "localhost")?.await?;
    println!("connected. enter your ID:");
    let mut my_id = String::new();
    io::stdin().read_line(&mut my_id)?;
    let my_id = my_id.trim().to_string();

    let shared_key = Key::from_slice(b"thorium_super_secret_key_32bytes");
    let cipher = ChaCha20Poly1305::new(shared_key);

    // BACKGROUND LISTENER
    let listen_conn = connection.clone();
    let listen_cipher = ChaCha20Poly1305::new(shared_key);
    tokio::spawn(async move {
        while let Ok((_, mut recv)) = listen_conn.accept_bi().await {
            let mut buf = vec![0; 2048];
            if let Ok(Some(n)) = recv.read(&mut buf).await {
                if let Ok(packet) = thorium::Packet::decode(&buf[..n]) {
                    if let Some(thorium::packet::Payload::Envelope(env)) = packet.payload {
                        let nonce = chacha20poly1305::Nonce::from_slice(&env.encrypted_payload[..12]);
                        let ciphertext = &env.encrypted_payload[12..];
                        if let Ok(decrypted) = listen_cipher.decrypt(nonce, ciphertext) {
                            println!("\n[FROM {}]: {}", env.sender_id, String::from_utf8_lossy(&decrypted));
                            print!("> "); io::stdout().flush().unwrap();
                        }
                    }
                }
            }
        }
    });

    println!("target ID to chat with:");
    let mut target_id = String::new();
    io::stdin().read_line(&mut target_id)?;
    let target_id = target_id.trim().to_string();

    loop {
        print!("> "); io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let text = input.trim();
        if text == "quit" { break; }

        let (mut send, mut recv_ack) = connection.open_bi().await?;
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&nonce, text.as_bytes()).unwrap();
        let mut payload = nonce.to_vec();
        payload.extend_from_slice(&ciphertext);

        let packet = thorium::Packet {
            payload: Some(thorium::packet::Payload::Envelope(thorium::QuicEnvelope {
                message_id: "1".into(),
                r#type: 1,
                timestamp: 0,
                sender_id: my_id.clone(),
                destination_id: target_id.clone(),
                encrypted_payload: payload,
            })),
        };

        let mut buf = Vec::new();
        packet.encode(&mut buf)?;
        send.write_all(&buf).await?;

        let mut ack_buf = vec![0; 1024];
        if let Ok(Some(n)) = recv_ack.read(&mut ack_buf).await {
            let res = thorium::Packet::decode(&ack_buf[..n])?;
            println!("status: {:?}", res.payload);
        }
    }
    Ok(())
}
