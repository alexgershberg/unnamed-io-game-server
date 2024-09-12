#[derive(Copy, Clone, Debug, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,

    pub(crate) max_x: f32,
    pub(crate) max_y: f32,
}

impl Velocity {
    pub fn to_be_bytes(&self) -> [u8; 8] {
        let x = f32::to_be_bytes(self.x);
        let y = f32::to_be_bytes(self.y);
        [x[0], x[1], x[2], x[3], y[0], y[1], y[2], y[3]]
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Acceleration {
    pub x: i8,
    pub y: i8,
}

#[cfg(test)]
mod tests {
    mod byte_order {

        use crate::physics::Velocity;

        #[ignore]
        #[test]
        fn velocity_byte_order() {
            let velocity = Velocity {
                x: -10.0,
                y: 25.0,
                ..Default::default()
            };
            assert_eq!(
                [
                    0b11000001, 0b00100000, 0b00000000, 0b00000000, 0b01000001, 0b11001000,
                    0b00000000, 0b00000000
                ],
                velocity.to_be_bytes()
            );
        }
    }
}
