use crate::config::Config;
use crate::net::packet::Packet;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::UdpSocket;

pub struct Client {
    pub socket: UdpSocket,
    target: SocketAddr,
}

impl Client {
    pub async fn new(config: Config) -> Self {
        let socket = match UdpSocket::bind((Ipv4Addr::new(127, 0, 0, 1), 0)).await {
            Ok(socket) => socket,
            Err(e) => panic!("Got error: {e}"),
        };
        println!("Created a socket: {socket:?}");

        Self {
            socket,
            target: config.addr,
        }
    }

    pub async fn send_packet(&mut self, packet: Packet) {
        let bytes = packet.to_bytes();
        let n = self
            .socket
            .send_to(bytes.as_slice(), self.target)
            .await
            .unwrap();
        println!("Packet: {packet:?}");
    }
}
