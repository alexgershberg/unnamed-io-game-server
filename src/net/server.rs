use crate::config::Config;
use crate::engine::Engine;
use crate::net::packet::{Move, Packet};
use std::io::Error;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, BufReader, BufWriter};
use tokio::net::tcp::ReadHalf;
use tokio::net::{TcpListener, TcpStream};

async fn read_command(reader: &mut BufReader<ReadHalf<'_>>) -> Result<Packet, Error> {
    let header = reader.read_u8().await?;
    match header {
        0x3 => {
            let id_0 = reader.read_u8().await?;
            let id_1 = reader.read_u8().await?;
            let dir = reader.read_u8().await?;

            Ok(Packet::Move(Move::from_bytes([id_0, id_1, dir])))
        }
        _ => todo!("Got header: {header}"),
    }
}

async fn handle_client_connection(mut stream: TcpStream, addr: SocketAddr) {
    let (rd, wr) = stream.split();
    let mut reader = BufReader::new(rd);
    let mut writer = BufWriter::new(wr);

    loop {
        let command = match read_command(&mut reader).await {
            Ok(command) => command,
            Err(e) => {
                println!("[{addr}]: Closing connection");
                break;
            }
        };

        println!("[{addr}]: {command:#?}")
    }
}

pub struct Server;

impl Server {
    pub async fn run(&self) {
        let config = Config::default();
        let server = TcpListener::bind((config.addr, config.port)).await.unwrap();

        let mut engine = Engine::default();
        tokio::spawn(async move { engine.run().await });

        loop {
            let client = server.accept().await;
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
