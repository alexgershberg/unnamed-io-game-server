use common::config::Config;
use common::id::Id;
use console::Term;
use net::client::Client;
use net::frame::Frame;
use net::packet::movement::Movement;
use net::packet::ping::Ping;
use net::packet::sync::Sync;
use net::packet::Packet;

#[tokio::main]
async fn main() {
    let term = Term::stdout();
    let id = 0;
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

        let mut frame = Frame::new();
        frame.packet = Some(packet);
        client.send_frame(frame).await;
    }
}
