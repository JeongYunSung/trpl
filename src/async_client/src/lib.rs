extern crate mini_redis;

mod channels;

pub use channels::*;

use mini_redis::{client};
use tokio::sync::mpsc::Receiver;

pub async fn connect(mut rx: Receiver<Command>) {
    let mut client = client::connect("127.0.0.1:6379").await.unwrap(); // connecting to the server in localhost

    while let Some(cmd) = rx.recv().await {
        use Command::*;

        match cmd {
            Get { key, resp } => {
                let res = client.get(&key).await;
                let _ = resp.send(res);
            }
            Set { key, val, resp } => {
                let res = client.set(&key, val).await;
                let _ = resp.send(res);
            }
        }
    }
}