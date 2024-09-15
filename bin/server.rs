use lib::config::Config;
use lib::engine::Engine;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::{channel, Sender};
use tokio::sync::Mutex;
use wasm_net::frame::Frame;
use wasm_net::packet::Packet;

#[derive(Debug)]
pub struct Connection {
    origin: SocketAddr,
    sequence_number: u32,
    acknowledgement_number: u32,
    pub engine_tx: Option<Arc<Sender<Packet>>>,
}

impl Connection {
    pub fn new(origin: SocketAddr) -> Self {
        Self {
            origin,
            sequence_number: 0,
            acknowledgement_number: 0,
            engine_tx: None,
        }
    }

    pub async fn handle_frame(&mut self, frame: Frame) {
        let Some(tx) = &mut self.engine_tx else {
            return;
        };

        let syn = frame.syn;
        if syn < self.acknowledgement_number {
            return;
        }

        self.acknowledgement_number = syn + 1;

        if let Some(packet) = frame.packet {
            tx.send(packet).await.unwrap();
        }
    }
}

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

#[tokio::main()]
async fn main() {
    let (engine_tx, server_rx) = channel(5);
    let mut engine = Engine {
        server_rx: Some(server_rx),
        ..Default::default()
    };
    let mut server = Server::from_config(Config {
        addr: "127.0.0.1:10001".parse().unwrap(),
    })
    .await;
    server.engine_tx = Some(Arc::new(engine_tx));

    let engine_handle = tokio::spawn(async move { engine.run().await });
    let server_handle = tokio::spawn(async move { server.run().await });
    let (engine_result, server_result) = tokio::join!(engine_handle, server_handle);
    engine_result.unwrap();
    server_result.unwrap();
}
