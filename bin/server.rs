use lib::config::Config;
use lib::engine::Engine;
use lib::net::server::Server;
use tokio::sync::mpsc::{channel, Receiver, Sender};

#[tokio::main()]
async fn main() {
    let (tx, rx) = channel(5);
    let mut engine = Engine {
        rx: Some(rx),
        ..Default::default()
    };
    let mut server = Server::from_config(Config {
        addr: "127.0.0.1:10001".parse().unwrap(),
    })
    .await;
    server.tx = Some(tx);

    let engine_handle = tokio::spawn(async move { engine.run().await });
    let server_handle = tokio::spawn(async move { server.run().await });
    let (engine_result, server_result) = tokio::join!(engine_handle, server_handle);
    engine_result.unwrap();
    server_result.unwrap();
}
