use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key,
};
use prost::Message;
use quinn::{ClientConfig, Endpoint};
use std::{error::Error, io::{self, Write}, sync::Arc};

pub mod thorium {
    include!(concat!(env!("OUT_DIR"), "/thorium.rs"));
}

struct SkipServerVerification;
impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let crypto = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(Arc::new(SkipServerVerification))
        .with_no_client_auth();
    let client_config = ClientConfig::new(Arc::new(crypto));

    let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
    endpoint.set_default_client_config(client_config);

    println!("trying to connect...");
    let connection = endpoint
        .connect("127.0.0.1:4433".parse()?, "localhost")?
        .await?;
    println!("connected. Type your message:");

    // Simulated MLS Shared Secret Key (Must be exactly 32 bytes)
    let shared_secret = b"thorium_super_secret_key_32bytes";
    let key = Key::from_slice(shared_secret);
    let cipher = ChaCha20Poly1305::new(key);

    let mut message_counter = 1;

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let text = input.trim();

        if text == "quit" { break; }
        if text.is_empty() { continue; }

        let (mut send_stream, mut recv_stream) = connection.open_bi().await?;

        // Cryptography: Generate a random 12-byte Nonce
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        
        // Cryptography: Encrypt the text
        let ciphertext = cipher.encrypt(&nonce, text.as_bytes())
            .expect("encryption failure");

        // Combine Nonce + Ciphertext for the payload
        let mut encrypted_payload = nonce.to_vec();
        encrypted_payload.extend_from_slice(&ciphertext);

        let envelope = thorium::QuicEnvelope {
            message_id: format!("msg-{}", message_counter),
            r#type: thorium::EventType::EventTextMessage as i32,
            timestamp: 1680000000,
            sender_id: "Ozymen".to_string(),
            destination_id: "Bob".to_string(),
            encrypted_payload,
        };
        message_counter += 1;

        let packet = thorium::Packet {
            payload: Some(thorium::packet::Payload::Envelope(envelope)),
        };

        let mut buffer = Vec::new();
        packet.encode(&mut buffer)?;
        send_stream.write_all(&buffer).await?;

        let mut recv_buffer = vec![0; 1024];
        if let Some(bytes_read) = recv_stream.read(&mut recv_buffer).await? {
            let response = thorium::Packet::decode(&recv_buffer[..bytes_read])?;
            if let Some(thorium::packet::Payload::Ack(ack)) = response.payload {
                println!("server response: Success = {}", ack.succes);
            }
        }

        send_stream.finish().await?;
    }

    connection.close(0u32.into(), b"End of test");
    Ok(())
}
