use crate::physics::Velocity;
use crate::player::{Id, Position};

#[derive(Copy, Clone, Debug, Default)]
pub struct Entity {
    id: Id,
    position: Position,
    velocity: Velocity,
}

impl Entity {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn tick(&mut self) {}
}
