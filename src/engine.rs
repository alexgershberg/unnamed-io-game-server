use crate::entity::Entity;
use crate::net::packet::Packet;
use crate::physics::{Acceleration, Velocity};
use crate::player::{Id, KeyboardInput, Player};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc::Receiver;
use tokio::time::{timeout, Instant};

pub const TPS: f32 = 20.0;

pub struct Engine {
    pub tps: f32,
    pub players: HashMap<Id, Player>,
    pub entities: Vec<Entity>,

    pub previous: Instant,
    pub lag: u128,

    pub server_rx: Option<Receiver<Packet>>,
}

impl Engine {
    pub async fn run(&mut self) {
        let ms_per_tick = (1000.0 / self.tps) as u128;
        loop {
            let now = Instant::now();
            let elapsed = self.previous.elapsed();

            self.input().await;
            self.lag += elapsed.as_millis();
            if self.lag >= ms_per_tick {
                self.tick();
                self.lag -= ms_per_tick;
            } else {
                tokio::time::sleep(Duration::from_millis((ms_per_tick - self.lag) as u64)).await;
            }

            self.previous = now;
        }
    }

    fn tick(&mut self) {
        for (_id, player) in self.players.iter_mut() {
            player.tick();
        }

        for entity in self.entities.iter_mut() {
            entity.tick()
        }
    }

    async fn input(&mut self) {
        if let Some(rx) = &mut self.server_rx {
            if let Ok(Some(packet)) = timeout(Duration::from_millis(1), rx.recv()).await {
                match packet {
                    Packet::Ping(_) => {}
                    Packet::Sync(_) => {}
                    Packet::Movement(movement) => {
                        let id = movement.id;
                        let keyboard_input = KeyboardInput {
                            up: movement.up,
                            down: movement.down,
                            left: movement.left,
                            right: movement.right,
                        };
                        if let Some(player) = self.players.get_mut(&id) {
                            player.input(keyboard_input)
                        }
                    }
                }
            }
        }
    }
}

fn create_n_entities(n: u16) -> Vec<Entity> {
    let mut output = vec![];
    for i in 0..n {
        output.push(Entity::new(Id(i)))
    }
    output
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            tps: TPS,
            players: {
                let mut player = Player::default();
                player.velocity = Velocity {
                    x: 0.0,
                    y: 0.0,
                    max_x: 10.0,
                    max_y: 10.0,
                };
                HashMap::from([(player.id, player)])
            },
            entities: create_n_entities(u16::MAX),
            previous: Instant::now(),
            lag: 0,
            server_rx: None,
        }
    }
}

#[test]
fn test() {
    let mut engine = Engine::default();
    let id = Id(0);
    let player = engine.players.get_mut(&id).unwrap();
    player.velocity = Velocity {
        x: 0.0,
        y: 0.0,
        max_x: 10.0,
        max_y: 10.0,
    };
    player.acceleration = Acceleration { x: 1, y: 0 };

    let tick = |engine: &mut Engine| {
        engine.tick();
        let player = engine.players.get_mut(&id).unwrap();
        println!(
            "||   {:7.prec$} | {:7.prec$}   ||   {:7.prec$} | {:7.prec$}   ||   {:2} | {:2}   ||",
            player.position.x,
            player.position.y,
            player.velocity.x,
            player.velocity.y,
            player.acceleration.x,
            player.acceleration.y,
            prec = 4,
        );
    };

    let steps = 20;
    for _ in 0..steps {
        tick(&mut engine);
        thread::sleep(Duration::from_millis((1000.0 / TPS) as u64))
    }

    let player = engine.players.get_mut(&id).unwrap();
    player.acceleration = Acceleration { x: -1, y: -1 };
    for _ in 0..steps {
        tick(&mut engine);
        thread::sleep(Duration::from_millis((1000.0 / TPS) as u64))
    }
}
