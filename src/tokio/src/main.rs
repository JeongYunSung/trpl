//! Tokio
//! 토키오는 비동기 처리 라이브러리로, 빠르고 안정적인 네트워크이다. ( 실제 star수 20K로 많은 편 )
//! 토키오는 하나의 작업에 대해 병렬처리하거나 Thread Pool을 이용해야 할 때를 제외하곤 사용할 수 있다
//! 토키오는 async-std와 달리 FFI 지원이 없다.

extern crate mini_redis;
extern crate tokio;

use mini_redis::{client, Result};

/// 런타임시점에 main을 호출하고 프로그램을 진행해야 하는데, main함수가 async면 호출되자마자 종료되기에 사용할 수 없음
/// #[tokio::main]은 tokio에서 제공하는 매크로로 async func에 붙일 시 자동으로 blocking을 처리해줌
#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?; // connecting to the server in localhost

    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
