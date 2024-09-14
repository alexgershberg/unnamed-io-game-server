use crate::net::packet::Packet;
use tokio_util::bytes::BufMut;

#[derive(Copy, Clone, Debug)]
pub struct Frame {
    pub version: u8,
    pub syn: u32,
    pub ack: u32,
    pub packet: Option<Packet>,
}

impl Frame {
    const PROTOCOL_VERSION: u8 = 1;

    pub fn new() -> Self {
        Self {
            version: Self::PROTOCOL_VERSION,
            syn: 0,
            ack: 0,
            packet: None,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut output = vec![self.version];
        let syn = self.syn.to_be_bytes();
        let ack = self.ack.to_be_bytes();
        output.put_slice(&syn);
        output.put_slice(&ack);

        if let Some(packet) = self.packet {
            let mut packet = match packet {
                Packet::Ping(ping) => ping.to_bytes(),
                Packet::Sync(sync) => sync.to_bytes(),
                Packet::Movement(movement) => movement.to_bytes(),
            };
            output.append(&mut packet);
        }

        output
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.is_empty() {
            return None;
        }

        let version = bytes[0];
        let syn = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
        let ack = u32::from_be_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]);
        let packet = Packet::from_bytes(&bytes[9..]);

        Some(Self {
            version,
            syn,
            ack,
            packet,
        })
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self::new()
    }
}
