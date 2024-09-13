use crate::config::Config;
use crate::net::packet::Packet;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct Client {
    pub stream: TcpStream,
}

impl Client {
    pub async fn new(config: Config) -> Self {
        let stream = match TcpStream::connect((config.addr, config.port)).await {
            Ok(stream) => stream,
            Err(e) => panic!("Got error: {e}"),
        };
        Self { stream }
    }

    pub async fn send_command(&mut self, command: Packet) {
        let bytes = command.to_bytes();
        self.stream.write_all(bytes.as_slice()).await.unwrap();
    }
}
