use crate::config::Config;
use crate::engine::Engine;
use crate::net::packet::{Move, Packet};
use std::io::Error;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, BufReader, BufWriter};
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::{TcpListener, TcpStream};

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

                Ok(Packet::Move(Move::from_bytes([id_0, id_1, dir])))
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

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn new() -> Self {
        Self::from_config(Config::default()).await
    }

    pub async fn from_config(config: Config) -> Self {
        let listener = TcpListener::bind((config.addr, config.port)).await.unwrap();
        Self { listener }
    }

    pub async fn run(&self) {
        let mut engine = Engine::default();
        tokio::spawn(async move { engine.run().await });

        loop {
            let client = self.listener.accept().await;
            match client {
                Ok((stream, addr)) => {
                    println!("[{addr}]: Accepting new connection");
                    tokio::spawn(async move { handle_client_connection(stream, addr).await });
                }
                Err(error) => {
                    eprintln!("Got an error: {error}")
                }
            }
        }
    }
}
