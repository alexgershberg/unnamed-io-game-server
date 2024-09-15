use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Copy, Clone, Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Sync;

impl Sync {
    pub const PACKET_ID: u8 = 1;

    pub fn to_bytes(&self) -> Vec<u8> {
        vec![Self::PACKET_ID]
    }
}
