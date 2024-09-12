use console::Term;
use lib::config::Config;
use lib::net::command::{Command, Move};
use lib::net::connection::Connection;
use lib::player::Id;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let config = Config::new();
    let mut connection = Connection::new(&config).await;

    let term = Term::stdout();
    loop {
        let (mut up, mut down, mut left, mut right) = (false, false, false, false);

        let char = term.read_char().unwrap();

        match char {
            'w' => up = true,
            's' => down = true,
            'a' => left = true,
            'd' => right = true,
            _ => {}
        };

        let command = Command::Move(Move {
            id: Id(5),
            up,
            down,
            left,
            right,
        });

        connection.send_command(command).await;
        sleep(Duration::from_millis(20)).await;
    }
}
