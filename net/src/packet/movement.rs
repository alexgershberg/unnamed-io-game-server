use crate::id::Id;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Copy, Clone, Debug)]
#[wasm_bindgen]
pub struct Movement {
    pub id: Id,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}
pub const MOVEMENT_PACKET_ID: u8 = 2;

#[wasm_bindgen]
impl Movement {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Movement {
        Movement {
            id: Default::default(),
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let up_flag: u8 = if self.up { 0b1000 } else { 0 };
        let down_flag: u8 = if self.down { 0b0100 } else { 0 };
        let left_flag: u8 = if self.left { 0b0010 } else { 0 };
        let right_flag: u8 = if self.right { 0b0001 } else { 0 };
        let flags = (up_flag | down_flag | left_flag | right_flag) & 0b1111;

        let mut id = self.id.0.to_be_bytes().to_vec();

        let mut output = vec![MOVEMENT_PACKET_ID];
        output.append(&mut id);
        output.push(flags);

        output
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Movement> {
        let packet_length = 3;
        if bytes.len() != packet_length {
            return None;
        }

        let id = Id(u16::from_be_bytes([bytes[0], bytes[1]]));

        let flags = bytes[2];
        let up = (flags & 0b1000) == 0b1000;
        let down = (flags & 0b0100) == 0b0100;
        let left = (flags & 0b0010) == 0b0010;
        let right = (flags & 0b0001) == 0b0001;
        Some(Self {
            id,
            up,
            down,
            left,
            right,
        })
    }
}
