use crate::config::Config;
use crate::net::connection::Connection;
use crate::net::frame::Frame;
use crate::net::packet::Packet;
use futures::lock::Mutex;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::Sender;

pub struct Server {
    socket: UdpSocket,
    pub engine_tx: Option<Arc<Sender<Packet>>>,
}

impl Server {
    pub async fn new() -> Self {
        Self::from_config(Config::default()).await
    }

    pub async fn from_config(config: Config) -> Self {
        let socket = UdpSocket::bind(config.addr).await.unwrap();
        println!("Creating socket: {socket:?}");
        Self {
            socket,
            engine_tx: None,
        }
    }

    pub async fn run(&self) {
        let mut connections: HashMap<SocketAddr, Arc<Mutex<Connection>>> = HashMap::new();

        loop {
            let mut buf = [0; 64];
            let (n, origin) = match self.socket.recv_from(&mut buf).await {
                Ok((n, origin)) => (n, origin),
                Err(e) => {
                    println!("Got error: {e}");
                    continue;
                }
            };

            let connection = connections.entry(origin).or_insert({
                let mut connection = Connection::new(origin);
                connection.engine_tx = self.engine_tx.clone();
                let connection = Arc::new(Mutex::new(connection));
                connection
            });

            let buf = &buf[..n];
            let frame = Frame::from_bytes(buf);
            if let Some(frame) = frame {
                let connection = connection.clone();
                tokio::spawn(async move {
                    let mut lock = connection.lock().await;
                    lock.handle_frame(frame).await;
                });
            }
        }
    }
}
