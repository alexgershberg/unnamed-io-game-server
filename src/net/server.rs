use crate::config::Config;
use crate::net::connection::Connection;
use crate::net::frame::Frame;
use futures::lock::Mutex;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::Sender;

pub struct Server {
    socket: UdpSocket,
    pub tx: Option<Sender<i32>>,
}

impl Server {
    pub async fn new() -> Self {
        Self::from_config(Config::default()).await
    }

    pub async fn from_config(config: Config) -> Self {
        let socket = UdpSocket::bind(config.addr).await.unwrap();
        println!("Creating socket: {socket:?}");
        Self { socket, tx: None }
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

            let p = origin.port();
            if let Some(tx) = &self.tx {
                tx.send(p as i32).await.unwrap();
            }

            let connection = connections
                .entry(origin)
                .or_insert(Arc::new(Mutex::new(Connection::new(origin))));

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
