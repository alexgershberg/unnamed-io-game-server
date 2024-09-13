#[derive(Copy, Clone, Debug)]
pub struct Ping;

impl Ping {
    pub const PACKET_ID: u8 = 0;

    pub fn as_bytes(&self) -> Vec<u8> {
        vec![Self::PACKET_ID]
    }
}
