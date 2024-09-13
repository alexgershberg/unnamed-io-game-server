#[derive(Copy, Clone, Debug)]
pub struct Sync;

impl Sync {
    pub const PACKET_ID: u8 = 1;

    pub fn as_bytes(&self) -> Vec<u8> {
        vec![Self::PACKET_ID]
    }
}
