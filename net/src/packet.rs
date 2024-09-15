use crate::packet::movement::Movement;
use crate::packet::ping::Ping;
use crate::packet::sync::Sync;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod movement;
pub mod ping;
pub mod sync;

#[derive(Copy, Clone, Debug, Tsify, Serialize, Deserialize)]
#[repr(u8)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Packet {
    Ping(Ping) = 0,
    Sync(Sync) = 1,
    Movement(Movement) = 2,
}

#[wasm_bindgen]
impl Packet {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Packet::Ping(ping) => ping.to_bytes(),
            Packet::Sync(sync) => sync.to_bytes(),
            Packet::Movement(movement) => movement.to_bytes().to_vec(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Packet> {
        if bytes.is_empty() {
            return None;
        }

        let packet_id = bytes[0];
        match packet_id {
            Ping::PACKET_ID => Some(Packet::Ping(Ping)),
            Sync::PACKET_ID => Some(Packet::Sync(Sync)),
            Movement::PACKET_ID => Some(Packet::Movement(Movement::from_bytes(&bytes[1..])?)),
            _ => todo!("Unknown packet id: {packet_id}"),
        }
    }
}

#[cfg(test)]
mod tests {
    mod byte_order {
        use crate::packet::movement::Movement;
        use common::id::Id;

        #[test]
        fn move_command_byte_order() {
            let command = Movement {
                id: Id(1),
                up: true,
                down: false,
                left: false,
                right: false,
            };
            let byte = command.to_bytes();
            assert_eq!(byte, [2, 0, 1, 0b1000]);
        }
    }
}
