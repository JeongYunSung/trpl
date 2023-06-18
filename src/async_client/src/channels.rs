//! Tokio의 채널은 총 4가지의 타입으로 제공한다.
//!
//! 1. mpsc : 다중 생산자 단일 소비자
//! 2. oneshot : 단일 생산자 단일 소비자
//! 3. broadcast : 단일 생산자 다중 소비자
//! 4. watch : 단일 생산자 다중 소비자

use bytes::Bytes;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    }
}

pub type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;