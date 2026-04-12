use quinn::{Endpoint, ServerConfig};
use std::{error::Error, net::SocketAddr, sync::Arc};

pub mod thorium {
	include!(concat!(env!("OUT_DIR"), "/thorium.rs"));
}
