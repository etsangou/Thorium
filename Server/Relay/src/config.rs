use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub network: NetworkConfig,
}

#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub ip: String,
    pub port: u16,
    pub cert_hostname: String,
}

#[derive(Deserialize, Clone)]
pub struct NetworkConfig {
    pub idle_timeout_secs: u64,
}

pub fn load() -> Config {
    let config_str = fs::read_to_string("config.toml")
        .expect("config.toml file missing");
    
    toml::from_str(&config_str)
        .expect("invalid config.toml syntax")
}
