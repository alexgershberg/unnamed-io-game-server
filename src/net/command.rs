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

    pub fn from_bytes(bytes: [u8; 3]) -> Self {
        let id = Id(u16::from_be_bytes([bytes[0], bytes[1]]));

        let dir = bytes[2];
        let up = (dir & 0b1000) == 0b1000;
        let down = (dir & 0b0100) == 0b0100;
        let left = (dir & 0b0010) == 0b0010;
        let right = (dir & 0b0001) == 0b0001;
        Self {
            id,
            up,
            down,
            left,
            right,
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Command {
    GetPlayer(GetPlayer) = 0x1,
    GetAllPlayers = 0x2,
    Move(Move) = 0x3,
}

impl Command {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Command::GetPlayer(_) => todo!(),
            Command::GetAllPlayers => todo!(),
            Command::Move(move_command) => {
                let bytes = move_command.as_bytes();
                vec![0x3, bytes[0], bytes[1], bytes[2]]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod byte_order {
        use crate::net::command::Move;
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
