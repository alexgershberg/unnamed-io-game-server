use crate::entity::Entity;
use crate::physics::{Acceleration, Velocity};
use crate::player::{Id, Player};
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc::Receiver;
use tokio::time::{timeout, Instant};

pub const TPS: f32 = 20.0;

pub struct Engine {
    pub tps: f32,
    pub players: Vec<Player>,
    pub entities: Vec<Entity>,

    pub previous: Instant,
    pub lag: u128,

    pub rx: Option<Receiver<i32>>,
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
        for player in self.players.iter_mut() {
            player.tick();
        }

        for entity in self.entities.iter_mut() {
            entity.tick()
        }
    }

    async fn input(&mut self) {
        if let Some(rx) = &mut self.rx {
            if let Ok(val) = timeout(Duration::from_millis(1000), rx.recv()).await {
                println!("[ENGINE]: Got val: {val:?}");
            } else {
                println!("[ENGINE]: receiver.recv timed out!");
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
            players: vec![Default::default()],
            entities: create_n_entities(u16::MAX),
            previous: Instant::now(),
            lag: 0,
            rx: None,
        }
    }
}

#[test]
fn test() {
    let mut engine = Engine::default();
    let player = &mut engine.players[0];
    player.velocity = Velocity {
        x: 0.0,
        y: 0.0,
        max_x: 10.0,
        max_y: 10.0,
    };
    player.acceleration = Acceleration { x: 1, y: 0 };

    let tick = |engine: &mut Engine| {
        engine.tick();
        let player = &mut engine.players[0];
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

    let player = &mut engine.players[0];
    player.acceleration = Acceleration { x: -1, y: -1 };
    for _ in 0..steps {
        tick(&mut engine);
        thread::sleep(Duration::from_millis((1000.0 / TPS) as u64))
    }
}
