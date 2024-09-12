use crate::config::Config;
use crate::net::packet::Packet;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct Client {
    pub stream: TcpStream,
}

impl Client {
    pub async fn new(config: &Config) -> Self {
        let stream = TcpStream::connect((config.addr, config.port))
            .await
            .unwrap();
        Self { stream }
    }

    pub async fn send_command(&mut self, command: Packet) {
        let bytes = command.to_bytes();
        self.stream.write_all(bytes.as_slice()).await.unwrap();
    }
}
