use serde::Deserialize;
use std::{
    fs::File,
    io::prelude::*,
    net::{SocketAddr, ToSocketAddrs},
};

const DEFAULT_HOSTNAME: &str = "::";
const DEFAULT_PORT: u16 = 50_066;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub ip: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug)]
pub enum Error {
    SocketAddrNotResolved,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::SocketAddrNotResolved => f.write_str("Socket address could not be resolved"),
        }
    }
}

impl std::error::Error for Error {}

impl Config {
    pub fn from_config_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config = toml::from_str(contents.as_str())?;
        Ok(config)
    }

    pub fn get_socket_addr(&self) -> Result<SocketAddr, Box<dyn std::error::Error>> {
        let ip = self
            .server
            .ip
            .as_ref()
            .map(|v| v.as_str())
            .unwrap_or(DEFAULT_HOSTNAME);
        let port = self.server.port.unwrap_or(DEFAULT_PORT);
        let sockt_addr = (ip, port)
            .to_socket_addrs()?
            .next()
            .ok_or(Error::SocketAddrNotResolved)?;
        Ok(sockt_addr)
    }
}
