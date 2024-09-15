use common::config::Config;
use lib::engine::Engine;
use net::server::Server;
use std::sync::Arc;
use tokio::sync::mpsc::channel;

#[tokio::main()]
async fn main() {
    let (engine_tx, server_rx) = channel(5);
    let mut engine = Engine {
        server_rx: Some(server_rx),
        ..Default::default()
    };
    let mut server = Server::from_config(Config {
        addr: "127.0.0.1:10001".parse().unwrap(),
    })
    .await;
    server.engine_tx = Some(Arc::new(engine_tx));

    let engine_handle = tokio::spawn(async move { engine.run().await });
    let server_handle = tokio::spawn(async move { server.run().await });
    let (engine_result, server_result) = tokio::join!(engine_handle, server_handle);
    engine_result.unwrap();
    server_result.unwrap();
}
