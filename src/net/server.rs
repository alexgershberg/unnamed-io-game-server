use crate::config::Config;
use crate::net::connection::Connection;
use crate::net::frame::Frame;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::UdpSocket;

pub struct Server {
    socket: UdpSocket,
}

impl Server {
    pub async fn new() -> Self {
        Self::from_config(Config::default()).await
    }

    pub async fn from_config(config: Config) -> Self {
        let socket = UdpSocket::bind(config.addr).await.unwrap();
        println!("Creating socket: {socket:?}");
        Self { socket }
    }

    pub async fn run(&self) {
        let mut connections: HashMap<SocketAddr, Connection> = HashMap::new();

        loop {
            let mut buf = [0; 64];
            let (n, origin) = match self.socket.recv_from(&mut buf).await {
                Ok((n, origin)) => (n, origin),
                Err(e) => {
                    println!("Got error: {e}");
                    continue;
                }
            };

            let connection = connections.entry(origin).or_insert(Connection::new(origin));

            let buf = &buf[..n];
            let frame = Frame::from_bytes(buf);
            if let Some(frame) = frame {
                connection.handle_frame(frame).await;
            }
        }
    }
}
