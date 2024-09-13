pub mod movement;
pub mod ping;
pub mod sync;

use crate::net::packet::movement::Movement;
use crate::net::packet::ping::Ping;
use crate::net::packet::sync::Sync;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Packet {
    Ping(Ping) = 0,
    Sync(Sync) = 1,
    Movement(Movement) = 2,
}

impl Packet {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Packet::Ping(ping) => ping.as_bytes(),
            Packet::Sync(sync) => sync.as_bytes(),
            Packet::Movement(movement) => movement.as_bytes().to_vec(),
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Option<Self> {
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
        use crate::net::packet::movement::Movement;
        use crate::player::Id;

        #[test]
        fn move_command_byte_order() {
            let command = Movement {
                id: Id(1),
                up: true,
                down: false,
                left: false,
                right: false,
            };
            let byte = command.as_bytes();
            assert_eq!(byte, [2, 0, 1, 0b1000]);
        }
    }
}
