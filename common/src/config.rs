use std::net::SocketAddr;

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub addr: SocketAddr,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1:10001".parse().unwrap(),
        }
    }
}
