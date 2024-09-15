use console::Term;
use lib::config::Config;
use net::frame::Frame;
use net::id::Id;
use net::packet::movement::Movement;
use net::packet::ping::Ping;
use net::packet::sync::Sync;
use net::packet::Packet;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::UdpSocket;

pub struct Client {
    pub socket: UdpSocket,
    target: SocketAddr,
    sequence_number: u32,
}

impl Client {
    pub async fn new(config: Config) -> Self {
        let socket = match UdpSocket::bind((Ipv4Addr::new(127, 0, 0, 1), 0)).await {
            Ok(socket) => socket,
            Err(e) => panic!("Got error: {e}"),
        };
        println!("Created a socket: {socket:?}");

        Self {
            socket,
            target: config.addr,
            sequence_number: 0,
        }
    }

    pub async fn send_frame(&mut self, mut frame: Frame) {
        frame.syn = self.sequence_number;
        self.sequence_number = self.sequence_number.wrapping_add(1);
        let bytes = frame.to_bytes();
        let n = self
            .socket
            .send_to(bytes.as_slice(), self.target)
            .await
            .unwrap();
        println!("Frame: {frame:?}");
    }
}

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
