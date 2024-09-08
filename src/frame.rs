#[derive(Copy, Clone, Debug, Default)]
pub struct Id(pub u16);

#[derive(Copy, Clone, Debug, Default)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

impl Position {
    pub fn to_be_bytes(&self) -> [u8; size_of::<Self>()] {
        let x = i16::to_be_bytes(self.x);
        let y = i16::to_be_bytes(self.y);
        [x[0], x[1], y[0], y[1]]
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Velocity {
    pub x: i8,
    pub y: i8,
}

impl Velocity {
    pub fn to_be_bytes(&self) -> [u8; size_of::<Self>()] {
        let x = i8::to_be_bytes(self.x);
        let y = i8::to_be_bytes(self.y);
        [x[0], y[0]]
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Player {
    pub id: Id,
    pub position: Position,
    pub velocity: Velocity,
}

impl Player {
    fn to_be_bytes(&self) -> [u8; size_of::<Self>()] {
        let id = self.id.0.to_be_bytes();
        let position = self.position.to_be_bytes();
        let velocity = self.velocity.to_be_bytes();
        [
            id[0],
            id[1],
            position[0],
            position[1],
            position[2],
            position[3],
            velocity[0],
            velocity[1],
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::frame::{Id, Player, Position, Velocity};

    #[test]
    fn byte_order() {
        let a: i32 = 0b00001111_00000111_00000011_00000001;
        assert_eq!(a, 252117761);
        assert_eq!([0b1111, 0b111, 0b11, 0b1], a.to_be_bytes());

        let b: i16 = 0b00001111_00000111;
        assert_eq!(b, 3847);
        assert_eq!([0b1111, 0b111], b.to_be_bytes());

        let c: i16 = 0b00000011_00000001;
        assert_eq!(c, 769);
        assert_eq!([0b11, 0b1], c.to_be_bytes());
    }

    #[test]
    fn player_byte_order() {
        let player = Player {
            id: Id(255),
            position: Position { x: 3847, y: 769 },
            velocity: Velocity { x: -10, y: 25 },
        };

        assert_eq!(
            [
                0b00000000, 0b11111111, 0b00001111, 0b00000111, 0b00000011, 0b00000001, 0b11110110,
                0b00011001
            ],
            player.to_be_bytes()
        );
    }

    #[test]
    fn position_byte_order() {
        let position = Position { x: 3847, y: 769 };
        assert_eq!(
            [0b00001111, 0b00000111, 0b_00000011, 0b00000001],
            position.to_be_bytes()
        );
    }

    #[test]
    fn mem_size() {
        let size = size_of::<Id>();
        assert_eq!(size, 2);

        let size = size_of::<Position>();
        assert_eq!(size, 4);

        let size = size_of::<Player>();
        assert_eq!(size, 6);
    }
}
