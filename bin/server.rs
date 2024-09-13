use lib::config::Config;
use lib::net::server::Server;

#[tokio::main()]
async fn main() {
    let server = Server::from_config(Config {
        addr: "127.0.0.1:10001".parse().unwrap(),
    })
    .await;
    server.run().await;
}
