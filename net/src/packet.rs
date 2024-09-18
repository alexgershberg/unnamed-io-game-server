use crate::packet::movement::Movement;
use crate::packet::ping::Ping;
use crate::packet::sync::Sync;

pub mod movement;
pub mod ping;
pub mod sync;

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
            ping::PING_PACKET_ID => Some(Packet::Ping(Ping)),
            sync::SYNC_PACKET_ID => Some(Packet::Sync(Sync)),
            movement::MOVEMENT_PACKET_ID => {
                Some(Packet::Movement(Movement::from_bytes(&bytes[1..])?))
            }
            _ => todo!("Unknown packet id: {packet_id}"),
        }
    }
}

#[cfg(test)]
mod tests {
    mod byte_order {
        use crate::id::Id;
        use crate::packet::movement::Movement;

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
