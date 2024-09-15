use wasm_net::id::Id;
use crate::engine::TPS;
use crate::physics::{Acceleration, Velocity};

pub struct KeyboardInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn to_be_bytes(&self) -> [u8; size_of::<Self>()] {
        let x = f32::to_be_bytes(self.x);
        let y = f32::to_be_bytes(self.y);
        // [x[0], x[1], y[0], y[1]];
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct Player {
    pub id: Id,
    pub position: Position,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
}

impl Player {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn tick(&mut self) {
        println!("[ENGINE]: {:#?}", self);
        self.update_position();
    }

    pub fn input(&mut self, keyboard_input: KeyboardInput) {
        let KeyboardInput {
            up,
            down,
            left,
            right,
        } = keyboard_input;

        self.acceleration.x = 0;
        self.acceleration.y = 0;

        if up {
            self.acceleration.y += 10;
        }

        if down {
            self.acceleration.y -= 10;
        }

        if left {
            self.acceleration.x -= 10;
        }

        if right {
            self.acceleration.x += 10;
        }
    }

    fn update_position(&mut self) {
        // if acceleration isn't 0, then friction comes into play

        self.velocity = self.calculate_velocity();
        self.position = self.calculate_position();
    }

    pub fn calculate_velocity(&self) -> Velocity {
        self.calculate_velocity_with_tps(TPS)
    }

    pub fn calculate_velocity_with_tps(&self, tps: f32) -> Velocity {
        let acceleration = self.acceleration;
        let dx = acceleration.x as f32 * (1.0 / tps);
        let dy = acceleration.y as f32 * (1.0 / tps);

        let mut velocity = self.velocity;
        increase_velocity(&mut velocity, dx, dy);

        velocity
    }

    pub fn calculate_position(&self) -> Position {
        self.calculate_position_with_tps(TPS)
    }

    pub fn calculate_position_with_tps(&self, tps: f32) -> Position {
        let velocity = self.velocity;
        let distance_x = velocity.x * (1.0 / tps);
        let distance_y = velocity.y * (1.0 / tps);

        let x = self.position.x + distance_x;
        let y = self.position.y + distance_y;

        Position { x, y }
    }

    fn to_be_bytes(
        &self,
    ) -> [u8; size_of::<Id>() + size_of::<Position>() + (size_of::<Velocity>() / 2)] {
        let id = self.id.as_bytes();
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
        ];
        todo!()
    }
}

fn increase_velocity(velocity: &mut Velocity, x: f32, y: f32) {
    if x > 0.0 {
        velocity.x = (velocity.x + x).min(velocity.max_x);
    } else {
        velocity.x = (velocity.x + x).max(-velocity.max_x);
    }

    if y > 0.0 {
        velocity.y = (velocity.y + y).min(velocity.max_y);
    } else {
        velocity.y = (velocity.y + y).max(-velocity.max_y);
    }
}

fn decrease_velocity(velocity: &mut Velocity, x: f32, y: f32) {
    if x > 0.0 {
        velocity.x = (velocity.x - x).max(0.0);
    } else {
        velocity.x = (velocity.x - x).min(0.0);
    }

    if y > 0.0 {
        velocity.y = (velocity.y - y).max(0.0);
    } else {
        velocity.y = (velocity.y - y).min(0.0);
    }
}
#[cfg(test)]
mod tests {
    mod byte_order {
        use crate::player::{Acceleration, Id, Player, Position, Velocity};

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

        #[ignore]
        #[test]
        fn player_byte_order() {
            let player = Player {
                id: Id(255),
                position: Position {
                    x: 3847.0,
                    y: 769.0,
                },
                velocity: Velocity {
                    x: -10.0,
                    y: 25.0,
                    ..Default::default()
                },
                acceleration: Acceleration { x: 5, y: 5 },
            };

            assert_eq!(
                [
                    0b00000000, 0b11111111, 0b00001111, 0b00000111, 0b00000011, 0b00000001,
                    0b11000001, 0b00100000, 0b00000000, 0b00000000, 0b01000001, 0b11001000,
                    0b00000000, 0b00000000, 255, 255, 255, 255 //TODO FIX THIS
                ],
                player.to_be_bytes()
            );
        }

        #[ignore]
        #[test]
        fn position_byte_order() {
            let position = Position {
                x: 3847.0,
                y: 769.0,
            };
            assert_eq!(
                [
                    0b00001111,
                    0b00000111,
                    0b_00000011,
                    0b00000001,
                    255,
                    255,
                    255,
                    255
                ],
                position.to_be_bytes()
            );
        }

        #[test]
        fn mem_size() {
            let size = size_of::<Id>();
            assert_eq!(size, 2);

            let size = size_of::<Position>();
            assert_eq!(size, 8);

            let size = size_of::<Player>();
            assert_eq!(size, 40);
        }
    }
    mod behavior {
        use crate::player::{decrease_velocity, increase_velocity, Id, Player, Position, Velocity};

        #[test]
        fn predict_position_with_tps_test() {
            let player = Player {
                id: Id(1),
                position: Position { x: 0.0, y: 0.0 },
                velocity: Velocity {
                    x: 10.0,
                    y: 5.0,
                    ..Default::default()
                },
                acceleration: Default::default(),
            };

            let tps = 20.0;
            let pos = player.calculate_position_with_tps(tps);
            assert_eq!(pos.x, 0.5);
            assert_eq!(pos.y, 0.25,);

            let tps = 10.0;
            let pos = player.calculate_position_with_tps(tps);
            assert_eq!(pos.x, 1.0,);
            assert_eq!(pos.y, 0.5,);

            let tps = 5.0;
            let pos = player.calculate_position_with_tps(tps);
            assert_eq!(pos.x, 2.0,);
            assert_eq!(pos.y, 1.0,);

            let tps = 1.0;
            let pos = player.calculate_position_with_tps(tps);
            assert_eq!(pos.x, 10.0,);
            assert_eq!(pos.y, 5.0,);

            let tps = 0.5;
            let pos = player.calculate_position_with_tps(tps);
            assert_eq!(pos.x, 20.0);
            assert_eq!(pos.y, 10.0);
        }

        #[test]
        fn calculate_position_with_negative_velocity() {
            let player = Player {
                velocity: Velocity {
                    x: -10.0,
                    y: 5.0,
                    ..Default::default()
                },
                ..Default::default()
            };
            let pos = player.calculate_position();
            assert_eq!(pos.x, -0.5);
            assert_eq!(pos.y, 0.25);
        }

        #[test]
        fn calculate_non_zero_position() {
            let player = Player {
                position: Position { x: 3.0, y: -0.25 },
                velocity: Velocity {
                    x: -10.0,
                    y: 5.0,
                    ..Default::default()
                },
                ..Default::default()
            };
            let pos = player.calculate_position();
            assert_eq!(pos.x, 2.5);
            assert_eq!(pos.y, 0.0);
        }

        #[test]
        fn increase_velocity_with_respect_to_max_test() {
            let mut velocity = Velocity {
                x: 9.8,
                y: -9.8,
                max_x: 10.0,
                max_y: 10.0,
            };

            increase_velocity(&mut velocity, 2.0, -2.0);
            assert_eq!(velocity.x, 10.0);
            assert_eq!(velocity.y, -10.0);
        }

        #[test]
        fn decrease_velocity_with_respect_to_max_test() {
            let mut velocity = Velocity {
                x: 0.8,
                y: -0.8,
                max_x: 10.0,
                max_y: 10.0,
            };

            decrease_velocity(&mut velocity, 2.0, -2.0);
            assert_eq!(velocity.x, 0.0);
            assert_eq!(velocity.y, 0.0);
        }
    }
}
