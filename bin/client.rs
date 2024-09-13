use console::Term;
use lib::config::Config;
use lib::net::client::Client;
use lib::net::packet::movement::Movement;
use lib::net::packet::ping::Ping;
use lib::net::packet::sync::Sync;
use lib::net::packet::Packet;
use lib::player::Id;

#[tokio::main]
async fn main() {
    let term = Term::stdout();
    let id = 9;
    let mut client = Client::new(Config {
        addr: "127.0.0.1:10001".parse().unwrap(),
    })
    .await;

    loop {
        let char = term.read_char().unwrap();
        let (mut up, mut down, mut left, mut right) = (false, false, false, false);
        let mut get_all_players = false;
        match char {
            'w' => up = true,
            's' => down = true,
            'a' => left = true,
            'd' => right = true,
            'p' => get_all_players = true,
            _ => {}
        };

        let mut packet = Packet::Ping(Ping);

        if get_all_players {
            packet = Packet::Sync(Sync);
        }
        if up || down || left || right {
            packet = Packet::Movement(Movement {
                id: Id(id as u16),
                up,
                down,
                left,
                right,
            });
        }

        client.send_packet(packet).await;
    }
}
