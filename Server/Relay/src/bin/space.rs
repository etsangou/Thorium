use prost::Message;
use quinn::{ClientConfig, Endpoint, TransportConfig};
use std::{error::Error, io::{self, Write}, sync::Arc, time::Duration, collections::HashMap};
use tokio::sync::Mutex;

pub mod thorium {
    include!(concat!(env!("OUT_DIR"), "/thorium.rs"));
}

struct SkipServerVerification;
impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(&self, _e: &rustls::Certificate, _i: &[rustls::Certificate], _s: &rustls::ServerName, _sc: &mut dyn Iterator<Item = &[u8]>, _oc: &[u8], _n: std::time::SystemTime) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}

type Salons = Arc<Mutex<HashMap<String, Vec<String>>>>;

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
    println!("space server ID:");
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

    let salons: Salons = Arc::new(Mutex::new(HashMap::new()));
    let listen_conn = connection.clone();
    let listen_salons = Arc::clone(&salons);
    let space_id = my_id.clone();

    tokio::spawn(async move {
        while let Ok((_, mut recv)) = listen_conn.accept_bi().await {
            let listen_conn_inner = listen_conn.clone();
            let space_id_inner = space_id.clone();
            let salons_inner = Arc::clone(&listen_salons);

            tokio::spawn(async move {
                let mut buf = vec![0; 2048];
                if let Ok(Some(n)) = recv.read(&mut buf).await {
                    if let Ok(packet) = thorium::Packet::decode(&buf[..n]) {
                        if let Some(thorium::packet::Payload::Envelope(env)) = packet.payload {
                            let channel = env.channel_id.clone();
                            if !channel.is_empty() {
                                let map = salons_inner.lock().await;
                                if let Some(members) = map.get(&channel) {
                                    let garbage = String::from_utf8_lossy(&env.encrypted_payload);
                                    println!("broadcasting on [{}] | payload: {}", channel, garbage);

                                    for member in members {
                                        if member != &env.sender_id {
                                            if let Ok((mut f_send, _)) = listen_conn_inner.open_bi().await {
                                                let mut new_env = env.clone();
                                                new_env.destination_id = member.clone();
                                                new_env.sender_id = space_id_inner.clone();

                                                let forward_packet = thorium::Packet {
                                                    payload: Some(thorium::packet::Payload::Envelope(new_env)),
                                                };
                                                let mut f_buf = Vec::new();
                                                forward_packet.encode(&mut f_buf).unwrap();
                                                let _ = f_send.write_all(&f_buf).await;
                                                let _ = f_send.finish().await;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
    });

    loop {
        println!("\n--- SPACE MENU ---");
        println!("1. Create salon");
        println!("2. Add user to salon");
        println!("3. List salons");
        print!("> "); io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                println!("salon name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name)?;
                let name = name.trim().to_string();
                salons.lock().await.insert(name.clone(), Vec::new());
                println!("salon {} created", name);
            }
            "2" => {
                println!("salon name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name)?;
                let name = name.trim().to_string();

                println!("user ID:");
                let mut user = String::new();
                io::stdin().read_line(&mut user)?;
                let user = user.trim().to_string();

                if let Some(members) = salons.lock().await.get_mut(&name) {
                    members.push(user.clone());
                    println!("user {} added to {}", user, name);
                } else {
                    println!("salon not found");
                }
            }
            "3" => {
                let map = salons.lock().await;
                for (k, v) in map.iter() {
                    println!("- {} : {:?}", k, v);
                }
            }
            _ => println!("invalid choice"),
        }
    }
}
