//! Future는 std library로 비동기의 결과값을 가져오기 위해 제공되는 Trait이다.
//!
//! Future은 poll함수를 통해 계산하고 값을 가져올 수 있다. 이 방식을 키워드로도 제공해주는데 await이다.
//!
//! 다른 언어의 비동기방식 또한 Rust의 Future과 같이 polling해오는 방식인거 같다.

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::{Duration, Instant};
use futures::future::poll_fn;
use tokio::sync::Notify;

pub struct Delay {
    pub when: Instant,
    pub waker: Option<Arc<Mutex<Waker>>>,
}

impl Future for Delay {
    type Output = &'static str;

    /// Waker란 해당 Task가 작업을 다시 실행할 수 있다고 외부에 알릴 수 있게 도와주는 구조체이다.
    /// Waker는 Task내에 Context에 등록되어 있어 Context를 활용해 가져올 수 있다.
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(waker) = &self.waker {
            let mut waker = waker.lock().unwrap();

            if !waker.will_wake(cx.waker()) {
                *waker = cx.waker().clone();
            }
        } else {
            let when = self.when;
            let waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(waker.clone());

            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }

                let waker = waker.lock().unwrap();
                waker.wake_by_ref();
            });
        }

        if Instant::now() >= self.when {
            println!("Future Doen");
            Poll::Ready("done")
        } else {
            println!("Future Pending");
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

/// poll_fn 클로저내에서 delay에 대해 poll을 한 뒤 tokio thread내 await에서 한번 더 poll이 진행된다.
/// 즉 각기다른 Waker인스턴스가 두 번이상 호출된다. 이 경우 가장 최근 polling에 대한 호출로 전달된 웨이커에서 wake를 호출해야 한다.
/// 항상 future를 구현할 때는 poll호출이 다른 웨이커 인스턴스를 제공할 수 있음을 가정해야 한다.
async fn double_polling_test() {
    let when = Instant::now() + Duration::from_millis(10);
    let mut delay = Some(Delay { when, waker: None });

    poll_fn(move |cx| {
        let mut delay = delay.take().unwrap();
        let res = Pin::new(&mut delay).poll(cx);
        assert!(res.is_pending());
        tokio::spawn(async move {
            delay.await;
        });

        Poll::Ready(())
    }).await;
}

/// 일반적으로 Waker를 사용하는건 Rust 비동기 API의 기반을 사용하는 것 이기에 어렵다.
/// 따라서 아래 예시와 같이 간단한 작업의 경우 Notify를 이용해 async/await을 구현할 수 있다.
/// notify는 notify_one()을 통해 신호를 알려주면 notified()를 통해 알림을 받을 수 있다.
async fn delay(dur: Duration) {
    let when = Instant::now() + dur;
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    thread::spawn(move || {
        let now = Instant::now();

        if now < when {
            thread::sleep(when - now);
        }

        notify2.notify_one();
    });

    notify.notified().await;
}