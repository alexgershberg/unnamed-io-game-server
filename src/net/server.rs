use crate::config::Config;
use crate::engine::Engine;
use crate::net::packet::Packet;
use std::io::Error;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, BufReader, BufWriter};
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::{TcpStream, UdpSocket};

struct Connection<'a> {
    addr: SocketAddr,
    reader: BufReader<ReadHalf<'a>>,
    writer: BufWriter<WriteHalf<'a>>,
}

impl<'a> Connection<'a> {
    async fn new(stream: &'a mut TcpStream, addr: SocketAddr) -> Self {
        let (rd, wr) = stream.split();
        let reader = BufReader::new(rd);
        let writer = BufWriter::new(wr);
        Self {
            addr,
            reader,
            writer,
        }
    }

    async fn read_command(&mut self) -> Result<Packet, Error> {
        let header = self.reader.read_u8().await?;
        match header {
            0x3 => {
                let id_0 = self.reader.read_u8().await?;
                let id_1 = self.reader.read_u8().await?;
                let dir = self.reader.read_u8().await?;

                // Ok(Packet::Move(Move::from_bytes([id_0, id_1, dir])))
                todo!()
            }
            _ => todo!("Got header: {header}"),
        }
    }
}

async fn handle_client_connection(mut stream: TcpStream, addr: SocketAddr) {
    let mut connection = Connection::new(&mut stream, addr).await;
    loop {
        let command = match connection.read_command().await {
            Ok(command) => command,
            Err(e) => {
                println!("[{addr}]: Closing connection");
                break;
            }
        };

        println!("[{addr}]: {command:#?}")
    }
}

async fn handle_packet(packet: Packet, addr: SocketAddr) {
    println!("[{addr}]: {packet:?}")
}

pub struct Server {
    socket: UdpSocket,
}

impl Server {
    pub async fn new() -> Self {
        Self::from_config(Config::default()).await
    }

    pub async fn from_config(config: Config) -> Self {
        let socket = UdpSocket::bind(config.addr).await.unwrap();
        println!("Creating socket: {socket:?}");
        Self { socket }
    }

    pub async fn run(&self) {
        let mut engine = Engine::default();
        tokio::spawn(async move { engine.run().await });

        loop {
            let mut buf = [0; 64];
            let (n, origin) = match self.socket.recv_from(&mut buf).await {
                Ok((n, addr)) => (n, addr),
                Err(e) => {
                    println!("Got error: {e}");
                    continue;
                }
            };

            let buf = &buf[..n];
            println!("n: {n} | origin: {origin:?} | buf: {buf:?}");
            let packet = Packet::from_bytes(buf.to_vec());
            println!("packet: {packet:?}");
            if let Some(packet) = packet {
                handle_packet(packet, origin);
            }
        }
    }
}
