use lib::config::Config;
use lib::engine::Engine;
use lib::net::server::Server;

#[tokio::main()]
async fn main() {
    let mut engine = Engine::default();
    let engine_handle = tokio::spawn(async move { engine.run().await });
    let server = Server::from_config(Config {
        addr: "127.0.0.1:10001".parse().unwrap(),
    })
    .await;
    let server_handle = tokio::spawn(async move { server.run().await });
    let (engine_result, server_result) = tokio::join!(engine_handle, server_handle);
    engine_result.unwrap();
    server_result.unwrap();
}
