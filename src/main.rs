mod frame;

use std::net::{Ipv4Addr, SocketAddr};
use tokio::io::{AsyncBufReadExt, BufReader, BufWriter};
use tokio::net::{TcpListener, TcpStream};

struct Config {
    addr: Ipv4Addr,
    port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            addr: Ipv4Addr::new(0, 0, 0, 0),
            port: 10001,
        }
    }
}

async fn handle_client_connection(mut stream: TcpStream, addr: SocketAddr) {
    let (rd, wr) = stream.split();
    let mut reader = BufReader::new(rd);
    let mut writer = BufWriter::new(wr);

    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line).await.unwrap();

        if n == 0 {
            eprintln!("n was 0");
            break;
        }
        println!("[{addr}]: {line:#?}")
    }
}

#[tokio::main]
async fn main() {
    let config = Config::default();
    let server = TcpListener::bind((config.addr, config.port)).await.unwrap();

    loop {
        let client = server.accept().await;
        match client {
            Ok((stream, addr)) => {
                println!("Accepting new connection: {}", &addr);
                tokio::spawn(async move { handle_client_connection(stream, addr).await });
            }
            Err(error) => {
                eprintln!("Got an error: {error}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::sink::SinkExt;
    use tokio_util::bytes::{BufMut, Bytes, BytesMut};
    use tokio_util::codec::{
        AnyDelimiterCodec, Decoder, FramedRead, FramedWrite, LengthDelimitedCodec, LinesCodec,
    };

    #[tokio::test]
    async fn test() {
        let buffer = Vec::new();
        let messages = vec!["Hello", "World"];
        // let messages = vec![Bytes::from("Hello"), Bytes::from("World")];
        // let encoder = LinesCodec::new();
        let mut encoder = AnyDelimiterCodec::new(b"\r\n".to_vec(), b"\n\r\n".to_vec());
        // let encoder = LengthDelimitedCodec::new();
        // let mut reader = FramedRead::new(buffer.clone(), encoder);
        let mut writer = FramedWrite::new(buffer.clone(), encoder.clone());
        writer.send(messages[0].clone()).await.unwrap();
        writer.send(messages[1].clone()).await.unwrap();
        let buf = writer.get_ref();
        let s = String::from_utf8(buf.clone()).unwrap();

        println!("{s:?}");
        println!("{buf:?}");

        let mut bytes = BytesMut::new();
        bytes.put_slice(b"hello\rworld\r\ne");
        let res = encoder.decode(&mut bytes);
        println!("{res:?}");
        let res = encoder.decode(&mut bytes);
        println!("{res:?}");
        let res = encoder.decode(&mut bytes);
        println!("{res:?}");
        let res = encoder.decode(&mut bytes);
        println!("{res:?}");
        let res = encoder.decode(&mut bytes);
        println!("{res:?}");
    }
}
