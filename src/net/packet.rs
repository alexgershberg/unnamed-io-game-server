use crate::player::Id;

#[derive(Copy, Clone, Debug)]
pub struct GetPlayer(pub Id);

#[derive(Copy, Clone, Debug)]
pub struct Move {
    pub id: Id,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Move {
    pub fn as_bytes(&self) -> [u8; 3] {
        let up_flag: u8 = if self.up { 0b1000 } else { 0 };
        let down_flag: u8 = if self.down { 0b0100 } else { 0 };
        let left_flag: u8 = if self.left { 0b0010 } else { 0 };
        let right_flag: u8 = if self.right { 0b0001 } else { 0 };
        let byte = (up_flag | down_flag | left_flag | right_flag) & 0b1111;

        let id = self.id.0.to_be_bytes();

        [id[0], id[1], byte]
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let packet_length = 3;
        if bytes.len() != packet_length {
            return None;
        }

        let id = Id(u16::from_be_bytes([bytes[0], bytes[1]]));

        let dir = bytes[2];
        let up = (dir & 0b1000) == 0b1000;
        let down = (dir & 0b0100) == 0b0100;
        let left = (dir & 0b0010) == 0b0010;
        let right = (dir & 0b0001) == 0b0001;
        Some(Self {
            id,
            up,
            down,
            left,
            right,
        })
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Packet {
    Ping = 0x0,
    GetPlayer(GetPlayer) = 0x1,
    GetAllPlayers = 0x2,
    Move(Move) = 0x3,
}

impl Packet {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Packet::Ping => vec![0x0],
            Packet::GetPlayer(_) => todo!(),
            Packet::GetAllPlayers => vec![0x2],
            Packet::Move(move_command) => {
                let bytes = move_command.as_bytes();
                vec![0x3, bytes[0], bytes[1], bytes[2]]
            }
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Option<Self> {
        if bytes.is_empty() {
            return None;
        }

        let header = bytes[0];
        match header {
            0x0 => Some(Packet::Ping),
            0x1 => todo!(),
            0x2 => Some(Packet::GetAllPlayers),
            0x3 => {
                let move_packet = Move::from_bytes(&bytes[1..])?;
                Some(Packet::Move(move_packet))
            }
            _ => todo!("Unknown packet header: {header}"),
        }
    }
}

#[cfg(test)]
mod tests {
    mod byte_order {
        use crate::net::packet::Move;
        use crate::player::Id;

        #[test]
        fn move_command_byte_order() {
            let command = Move {
                id: Id(1),
                up: true,
                down: false,
                left: false,
                right: false,
            };
            let byte = command.as_bytes();
            assert_eq!(byte, [0, 1, 0b1000]);
        }
    }
}
