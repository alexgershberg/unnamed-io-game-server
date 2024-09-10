use crate::player::{Id, Player};
use std::time::Duration;
use tokio::time::Instant;

pub struct Engine {
    pub tps: f32,
    players: Vec<Player>,

    previous: Instant,
    lag: u128,
}

impl Engine {
    pub async fn run(&mut self) {
        let ms_per_tick = (1000.0 / self.tps) as u128;
        loop {
            let now = Instant::now();
            let elapsed = self.previous.elapsed();

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
        for player in &mut self.players {
            player.tick()
        }
    }
}

fn create_n_players(n: u16) -> Vec<Player> {
    let mut output = vec![];
    for i in 0..n {
        output.push(Player::new(Id(i)))
    }
    output
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            tps: 20.0,
            players: create_n_players(65535),
            previous: Instant::now(),
            lag: 0,
        }
    }
}
