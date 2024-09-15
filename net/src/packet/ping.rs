use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

#[derive(Copy, Clone, Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Ping;

impl Ping {
    pub const PACKET_ID: u8 = 0;

    pub fn to_bytes(&self) -> Vec<u8> {
        vec![Self::PACKET_ID]
    }
}
