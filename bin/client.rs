use lib::config::Config;
use lib::net::client::Client;
use lib::net::packet::{Move, Packet};
use lib::player::Id;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let mut handles = vec![];
    for id in 1..1000 {
        println!("id: {id}");
        let handle = tokio::spawn(async move {
            let mut connection = Client::new(Config::new()).await;
            loop {
                let (up, down, left, right) = (false, false, false, false);

                // let char = term.read_char().unwrap();
                //
                // match char {
                //     'w' => up = true,
                //     's' => down = true,
                //     'a' => left = true,
                //     'd' => right = true,
                //     _ => {}
                // };

                let command = Packet::Move(Move {
                    id: Id(id as u16),
                    up,
                    down,
                    left,
                    right,
                });

                connection.send_command(command).await;
                sleep(Duration::from_millis(10 * id)).await;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap()
    }
}
