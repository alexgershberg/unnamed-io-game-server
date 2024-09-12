use tokio::io::{AsyncBufReadExt, AsyncReadExt};
use tokio::runtime;
use lib::net::server::Server;

fn main() {
    let rt = runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .unwrap();
    rt.block_on(async {
        let server = Server;
        server.run().await;
    });
}

#[cfg(test)]
mod tests {
    use futures::sink::SinkExt;
    use tokio_util::bytes::{BufMut, BytesMut};
    use tokio_util::codec::{AnyDelimiterCodec, Decoder, FramedWrite};

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
