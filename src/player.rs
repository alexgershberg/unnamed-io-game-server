use crate::engine::TPS;
use tokio::time::Instant;

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
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn to_be_bytes(&self) -> [u8; size_of::<Self>()] {
        let x = f32::to_be_bytes(self.x);
        let y = f32::to_be_bytes(self.y);
        [x[0], x[1], x[2], x[3], y[0], y[1], y[2], y[3]]
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Acceleration {
    pub x: u8,
    pub y: u8,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub id: Id,
    pub position: Position,
    pub velocity: Velocity,
    pub acceleration: Acceleration,

    pub(crate) previous: Instant,
    pub counter: u32,
}

impl Player {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            ..Self::default()
        }
    }
    pub(crate) fn tick(&mut self) {
        let now = Instant::now();
        if self.counter >= 10 && self.id.0 == 40000 {
            let elapsed = self.previous.elapsed();
            println!(
                "{} | player since last tick: {}",
                self.id.0,
                elapsed.as_millis()
            );
            self.counter = 0;
        }
        self.counter += 1;
        self.previous = now;

        self.update_position();
    }

    fn input(&self) {}

    fn update_position(&mut self) {
        let (distance_x, distance_y) = self.predict_position();
        self.position.x = 0;
        self.position.y = 0;
    }

    pub fn predict_position(&self) -> (f32, f32) {
        self.predict_position_with_tps(TPS)
    }

    pub fn predict_position_with_tps(&self, tps: f32,) -> (f32, f32) {
        let velocity = self.velocity;
        let distance_x = velocity.x * (1.0 / tps);
        let distance_y = velocity.y * (1.0 / tps);
        (distance_x, distance_y)
    }

    fn to_be_bytes(&self) -> [u8; size_of::<Id>() + size_of::<Position>() + size_of::<Velocity>()] {
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
            velocity[2],
            velocity[3],
            velocity[4],
            velocity[5],
            velocity[6],
            velocity[7],
        ]
    }
}
impl Default for Player {
    fn default() -> Self {
        Self {
            id: Id::default(),
            position: Position::default(),
            velocity: Velocity::default(),
            acceleration: Acceleration::default(),
            previous: Instant::now(),
            counter: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    mod byte_order {
        use crate::player::{Acceleration, Id, Player, Position, Velocity};
        use tokio::time::Instant;

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
                velocity: Velocity { x: -10.0, y: 25.0 },
                acceleration: Acceleration { x: 5, y: 5 },
                previous: Instant::now(),
                counter: 0,
            };

            assert_eq!(
                [
                    0b00000000, 0b11111111, 0b00001111, 0b00000111, 0b00000011, 0b00000001,
                    0b11000001, 0b00100000, 0b00000000, 0b00000000, 0b01000001, 0b11001000,
                    0b00000000, 0b00000000,
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
        fn velocity_byte_order() {
            let velocity = Velocity { x: -10.0, y: 25.0 };
            assert_eq!(
                [
                    0b11000001, 0b00100000, 0b00000000, 0b00000000, 0b01000001, 0b11001000,
                    0b00000000, 0b00000000
                ],
                velocity.to_be_bytes()
            );
        }

        #[test]
        fn mem_size() {
            let size = size_of::<Id>();
            assert_eq!(size, 2);

            let size = size_of::<Position>();
            assert_eq!(size, 4);

            let size = size_of::<Player>();
            assert_eq!(size, 40);
        }
    }
    mod behavior {
        use crate::player::{Id, Player, Position, Velocity};
        use tokio::time::Instant;

        #[test]
        fn predict_position_with_tps_test() {

            let player = Player {
                id: Id(1),
                position: Position { x: 0, y: 0 },
                velocity: Velocity { x: 10.0, y: 5.0 },
                acceleration: Default::default(),

                previous: Instant::now(),
                counter: 0,
            };

            let tps = 20.0;
            let (dist_x, dist_y) = player.predict_position_with_tps(tps);
            assert_eq!(0.5, dist_x);
            assert_eq!(0.25, dist_y);

            let tps = 10.0;
            let (dist_x, dist_y) = player.predict_position_with_tps(tps);
            assert_eq!(1.0, dist_x);
            assert_eq!(0.5, dist_y);

            let tps = 5.0;
            let (dist_x, dist_y) = player.predict_position_with_tps(tps);
            assert_eq!(2.0, dist_x);
            assert_eq!(1.0, dist_y);

            let tps = 1.0;
            let (dist_x, dist_y) = player.predict_position_with_tps(tps);
            assert_eq!(10.0, dist_x);
            assert_eq!(5.0, dist_y);

            let tps = 0.5;
            let (dist_x, dist_y) = player.predict_position_with_tps(tps);
            assert_eq!(20.0, dist_x);
            assert_eq!(10.0, dist_y);
        }
    }
}
