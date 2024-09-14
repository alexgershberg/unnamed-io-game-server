use crate::config::Config;
use crate::net::frame::Frame;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::UdpSocket;

pub struct Client {
    pub socket: UdpSocket,
    target: SocketAddr,
    sequence_number: u32,
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
            sequence_number: 0,
        }
    }

    pub async fn send_frame(&mut self, mut frame: Frame) {
        frame.syn = self.sequence_number;
        self.sequence_number = self.sequence_number.wrapping_add(1);
        let bytes = frame.to_bytes();
        let n = self
            .socket
            .send_to(bytes.as_slice(), self.target)
            .await
            .unwrap();
        println!("Frame: {frame:?}");
    }
}
