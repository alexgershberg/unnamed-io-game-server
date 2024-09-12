use crate::config::Config;
use crate::net::command::Command;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct Connection {
    pub stream: TcpStream,
}

impl Connection {
    pub async fn new(config: &Config) -> Self {
        let stream = TcpStream::connect((config.addr, config.port))
            .await
            .unwrap();
        Self { stream }
    }

    pub async fn send_command(&mut self, command: Command) {
        let bytes = command.to_bytes();
        self.stream.write_all(bytes.as_slice()).await.unwrap();
    }
}
