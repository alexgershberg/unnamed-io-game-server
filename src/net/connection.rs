use crate::net::frame::Frame;
use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Connection {
    origin: SocketAddr,
    sequence_number: u32,
    acknowledgement_number: u32,
}

impl Connection {
    pub fn new(origin: SocketAddr) -> Self {
        Self {
            origin,
            sequence_number: 0,
            acknowledgement_number: 0,
        }
    }

    pub async fn handle_frame(&mut self, frame: Frame) {
        let syn = frame.syn;
        if syn < self.acknowledgement_number {
            return;
        }

        self.acknowledgement_number = syn + 1;
        println!(
            "[{}]: frame.syn: {} | frame.ack: {} | self.syn: {} | self.ack: {}",
            self.origin, frame.syn, frame.ack, self.sequence_number, self.acknowledgement_number
        )
    }
}
