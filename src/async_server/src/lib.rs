//! Tokio
//! 토키오는 비동기 처리 라이브러리로, 빠르고 안정적인 네트워크이다. ( 실제 star수 20K로 많은 편 )
//! 토키오는 하나의 작업에 대해 병렬처리하거나 Thread Pool을 이용해야 할 때를 제외하곤 사용할 수 있다
//! 토키오는 async_server-std와 달리 FFI 지원이 없다.

extern crate tokio;
extern crate mini_redis;

use bytes::Bytes;
use std::collections::HashMap;
use tokio::{task, task::yield_now};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

pub async fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let db = db.clone();

        // GreenThread 생성
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        let response= match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                println!("SET : {} {:?}", cmd.key(),cmd.value());
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                println!("GET : {} {:?}", cmd.key(), db.get(cmd.key()));
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}

/// Tokio::spawn은 비동기 블록을 전달하고 JoinHandle을 반환한다
/// JoinHandle을 wait시키면 Result가 반환된다.
/// JoinHandle은 내부적으로 비동기 블록(future)를 Task로 패키징되며, Task는 Tokio Scheduler에 의해 관리된다.
/// 이 때 Task는 spawn된곳과 동일한 스레드 혹은 다른 스레드에 의해 실행된다.
/// Task는 매우 가벼우며 단 하나의 할당과 64바이트의 메모리만 사용하여, 최대 수천개의 Task를 자유롭게 생성할 수 있다.
pub async fn spawn_task() {
    let handle = tokio::spawn(async {
        "return value"
    });

    let out = handle.await.unwrap();
    println!("out = {}", out);
}

/// Tokio의 Task의 경우 trait bound에 'static설정이 되어있다.
/// 즉 task가 언제까지 사용될지 모르기에 해당 Type에 대해 프로그램 전체생애주기에 보장을 한다.
pub async fn task_static() {
    let v = vec![1, 2, 3];

    task::spawn(async move {
        println!("Here's a vec: {:?}", v);
    });
}

/// yield는 tokio runtime으로 스레드를 전환하고 task queue에 현재 task를 enqueue한다.
/// tokio runtime에서는 scheduler가 task queue에서 task를 dequeue하고 실행한다.
/// 이 때 A Thread에서 반환한 Task B Thread에서 처리할 수 있는데, 이를 지원하기 위해 Send Trait를 구현해야 한다.
/// 즉 해당 Task에서 사용하는 개체들이 Send Trait을 지원해야 하며 지원하지 않을 시 에러가 발생한다
/// ```
/// tokio::spawn(async {
///     let rc = Rc::new("hello");
///     println!("{}", rc);
///     yield_now().await;
/// });
/// ```
/// 가령 위와같이 Rc(Trait을 구현하지 않음)의 스코프를 강제로 닫아 해제해주지 않고 사용한다면, Send Trait을 구현하지 않았기에 에러가 발생한다.
pub async fn yield_and_send() {
    tokio::spawn(async {
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }

        yield_now().await;
    });
}