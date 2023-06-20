//! Executors
//!
//! 비동기함수는 tokio::spawn 혹은 #[tokio::main]이 달린 main함수에서 실행이 된다.
//! 또한 future는 future를 포함할 수 있고, await이 호출되는 시점에 polling을 하게 된다.
//! 그렇다면 가장 최상위에 있는 async function을 polling해주는 것은 누구일까 ? 그 역할이 바로 tokio executors이다.

use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::time::{Duration, Instant};
use futures::task;
use super::Delay;

pub fn execute() {
    let mut mini_tokio = MiniTokio::new();

    println!("before");

    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_millis(10);
        let future = Delay { when };

        let out = future.await;
        assert_eq!(out, "done");
    });

    println!("middle");

    mini_tokio.run();
    println!("after");
}

struct MiniTokio {
    tasks: VecDeque<Task>,
}

type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

impl MiniTokio {
    fn new() -> MiniTokio {
        MiniTokio {
            tasks: VecDeque::new(),
        }
    }

    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.tasks.push_back(Box::pin(future));
    }

    fn run(&mut self) {
        let waker = task::noop_waker();
        let mut cx = Context::from_waker(&waker);

        while let Some(mut task) = self.tasks.pop_front() {
            if task.as_mut().poll(&mut cx).is_pending() {
                self.tasks.push_back(task);
            }
        }
    }
}