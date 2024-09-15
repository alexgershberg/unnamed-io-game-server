use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Copy, Clone, Debug)]
#[wasm_bindgen]
pub struct Sync;
pub const SYNC_PACKET_ID: u8 = 1;

#[wasm_bindgen]
impl Sync {
    pub fn to_bytes(&self) -> Vec<u8> {
        vec![SYNC_PACKET_ID]
    }
}
