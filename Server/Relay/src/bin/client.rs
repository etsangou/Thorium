use prost::Message;
use quinn::{ClientConfig, Endpoint};
use std::{error::Error, sync::Arc};

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
    println!("connected");

    let (mut send_stream, mut recv_stream) = connection.open_bi().await?;

    let envelope = thorium::QuicEnvelope {
        message_id: "test-1234-abcd".to_string(),
        type_: thorium::EventType::EventTextMessage as i32,
        timestamp: 1680000000,
        sender_id: "sender_Terminal".to_string(),
        destination_id: "receiver_ID".to_string(),
        encrypted_payload: b"TEST".to_vec(),
    };

    let packet = thorium::ThoriumPacket {
        payload: Some(thorium::thorium_packet::Payload::Envelope(envelope)),
    };

    let mut buffer = Vec::new();
    packet.encode(&mut buffer)?;
    
    println!("Sending {} bytes...", buffer.len());
    send_stream.write_all(&buffer).await?;

    let mut recv_buffer = vec![0; 1024];
    if let Some(bytes_read) = recv_stream.read(&mut recv_buffer).await? {
    
        let response = thorium::ThoriumPacket::decode(&recv_buffer[..bytes_read])?;
        
        match response.payload {
            Some(thorium::thorium_packet::Payload::Ack(ack)) => {
                println!("server response: Success = {}, Msg = {}", ack.success, ack.error_message);
            }
            _ => println!("received only ACK"),
        }
    }

    send_stream.finish().await?;
    connection.close(0u32.into(), b"End of test");
    
    Ok(())
}
