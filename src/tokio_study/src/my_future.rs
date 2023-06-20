//! Future는 std library로 비동기의 결과값을 가져오기 위해 제공되는 Trait이다.
//!
//! Future은 poll함수를 통해 계산하고 값을 가져올 수 있다. 이 방식을 키워드로도 제공해주는데 await이다.
//!
//! 다른 언어의 비동기방식 또한 Rust의 Future과 같이 polling해오는 방식인거 같다.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

pub struct Delay {
    pub when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}