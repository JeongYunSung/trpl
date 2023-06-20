//! Future는 std library로 비동기의 결과값을 가져오기 위해 제공되는 Trait이다.
//!
//! Future은 poll함수를 통해 계산하고 값을 가져올 수 있다. 이 방식을 키워드로도 제공해주는데 await이다.
//!
//! 다른 언어의 비동기방식 또한 Rust의 Future과 같이 polling해오는 방식인거 같다.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::Instant;

pub struct Delay {
    pub when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    /// Waker란 해당 Task가 작업을 다시 실행할 수 있다고 외부에 알릴 수 있게 도와주는 구조체이다.
    /// Waker는 Task내에 Context에 등록되어 있어 Context를 활용해 가져올 수 있다.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            let waker = cx.waker().clone();
            let when = self.when;

            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);

                    waker.wake();
                }
            });

            Poll::Pending
        }
    }
}

impl Delay {

    /// Waker를 사용하지 않은 개선되기 이전에 poll
    fn not_optimized_poll(self: Pin<&mut Self>, cx: &mut Context<'_>) {
        if Instant::now() >= self.when {
            println!("Hello world");
            let _ = Poll::Ready("done");
        } else {
            // 아래 wake_by_ref는 효과적이고 암묵적이게 소유한 Waker를 이용해서 wake처리를 한다.
            // 다만 이는 필요이상의 CPU사이클이 낭비될 수 있다.
            cx.waker().wake_by_ref();
            // Poll::Pending;
        }
    }
}