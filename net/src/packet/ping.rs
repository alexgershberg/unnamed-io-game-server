use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Copy, Clone, Debug)]
#[wasm_bindgen]
pub struct Ping;

pub const PING_PACKET_ID: u8 = 0;

#[wasm_bindgen]
impl Ping {
    pub fn to_bytes(&self) -> Vec<u8> {
        vec![PING_PACKET_ID]
    }
}
