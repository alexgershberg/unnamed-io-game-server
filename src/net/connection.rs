use crate::net::frame::Frame;
use crate::net::packet::Packet;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

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
