use async_client::*;
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    let manager = tokio::spawn(async move {
        connect(rx).await;
    });

    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };

        tx.send(cmd).await.unwrap();

        let res = resp_rx.await;
        println!("Get = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        };

        tx2.send(cmd).await.unwrap();

        let res = resp_rx.await;
        println!("Set = {:?}", res);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}

