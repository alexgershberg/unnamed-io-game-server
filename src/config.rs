use std::net::Ipv4Addr;

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub addr: Ipv4Addr,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            addr: Ipv4Addr::new(127, 0, 0, 1),
            port: 10001,
        }
    }
}
