extern crate tokio;

use echo_server::open_server;

#[tokio::main]
async fn main() {
    let _ = open_server().await;
}