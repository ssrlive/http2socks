use serde_derive::{Deserialize, Serialize};
use std::net::SocketAddr;

/// Proxy tunnel from HTTP to SOCKS5
#[derive(Debug, Clone, clap::Parser, Serialize, Deserialize)]
#[command(author, version, about = "http2socks application.", long_about = None)]
pub struct Config {
    /// Local listening address
    #[arg(short, long, value_name = "IP:port")]
    pub local_addr: SocketAddr,

    /// Remote SOCKS5 server address
    #[arg(short, long, value_name = "IP:port")]
    pub server_addr: SocketAddr,

    /// HTTP client authentication username
    #[arg(short, long, value_name = "username")]
    pub username: Option<String>,

    /// HTTP client authentication password
    #[arg(short, long, value_name = "password")]
    pub password: Option<String>,

    /// Log verbosity level
    #[arg(short, long, value_name = "level", default_value = "info")]
    pub verbosity: ArgVerbosity,
}

impl Default for Config {
    fn default() -> Self {
        use clap::Parser;
        Self::parse()
    }
}

impl Config {
    pub fn new(local_addr: SocketAddr, server_addr: SocketAddr) -> Self {
        Config {
            local_addr,
            server_addr,
            username: None,
            password: None,
            verbosity: ArgVerbosity::Info,
        }
    }

    pub fn username(&mut self, username: &str) -> &mut Self {
        self.username = Some(username.to_string());
        self
    }

    pub fn password(&mut self, password: &str) -> &mut Self {
        self.password = Some(password.to_string());
        self
    }

    pub fn verbosity(&mut self, verbosity: ArgVerbosity) -> &mut Self {
        self.verbosity = verbosity;
        self
    }

    pub fn get_credentials(&self) -> Credentials {
        Credentials {
            username: self.username.clone(),
            password: self.password.clone(),
        }
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, clap::ValueEnum, Serialize, Deserialize)]
pub enum ArgVerbosity {
    Off,
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

impl std::fmt::Display for ArgVerbosity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ArgVerbosity::Off => write!(f, "off"),
            ArgVerbosity::Error => write!(f, "error"),
            ArgVerbosity::Warn => write!(f, "warn"),
            ArgVerbosity::Info => write!(f, "info"),
            ArgVerbosity::Debug => write!(f, "debug"),
            ArgVerbosity::Trace => write!(f, "trace"),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Credentials {
    pub fn new(username: &str, password: &str) -> Self {
        Credentials {
            username: Some(username.to_string()),
            password: Some(password.to_string()),
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let empty = "".to_owned();
        let u = self.username.as_ref().unwrap_or(&empty);
        let p = self.password.as_ref().unwrap_or(&empty);
        format!("{}:{}", u, p).as_bytes().to_vec()
    }

    pub fn is_empty(&self) -> bool {
        self.to_vec() == b":".to_vec()
    }
}
