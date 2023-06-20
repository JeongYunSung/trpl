//! Executors
//!
//! 비동기함수는 tokio::spawn 혹은 #[tokio::main]이 달린 main함수에서 실행이 된다.
//! 또한 future는 future를 포함할 수 있고, await이 호출되는 시점에 polling을 하게 된다.
//! 그렇다면 가장 최상위에 있는 async function을 polling해주는 것은 누구일까 ? 그 역할이 바로 tokio executors이다.

use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::Context;
use std::time::{Duration, Instant};
use crossbeam::channel;
use futures::task;
use futures::task::ArcWake;
use super::Delay;

pub fn execute() {
    let mut mini_tokio = MiniTokio::new();

    println!("before");

    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_millis(10);
        let future = Delay { when, waker: None };

        let out = future.await;
        assert_eq!(out, "done");
        println!("Result : {}", out);
    });

    println!("middle");

    mini_tokio.run();
    println!("after");
}

struct MiniTokio {
    scheduled: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>,
}

struct Task {
    // Task에 대해 Sync를 구현하기 위해 Mutex를 사용
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    executor: channel::Sender<Arc<Task>>,
}

impl Task {
    fn schedule(self: &Arc<Self>) {
        let _ = self.executor.send(self.clone());
    }

    fn poll(self: Arc<Self>) {
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);

        let mut future = self.future.try_lock().unwrap();

        let _ = future.as_mut().poll(&mut cx);
    }

    fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });

        let _ = sender.send(task);
    }
}

// Waker를 hooking하기 위해선 low level API인 vtable을 사용해야 한다. RawWakerVTable
// RawWakerVTable은 쉽게 어떤 Waker를 참조하고 있는지 vtable로서 제공해주는 구조체이다.
// 이 부분은 핸들링 하기엔 손이 많이 가기에 futures의 ArcWake Trait을 사용한다.
impl ArcWake for Task {

    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}

impl MiniTokio {
    fn new() -> MiniTokio {
        let (sender, scheduled) = channel::unbounded();

        MiniTokio { scheduled, sender }
    }

    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task::spawn(future, &self.sender);
    }

    /// Waker방식을 적용할 때 Channel을 이용할 것 이다.
    /// 하지만 std channel의 경우 sync가 구현이 되어있지 않아 사용할 수 없다.
    /// 따라서 crossbeam의 channel을 이용해야 한다.
    fn run(&self) {
        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }
}

type OldTask = Pin<Box<dyn Future<Output = ()> + Send>>;

struct OldMiniTokio {
    tasks: VecDeque<OldTask>,
}

impl OldMiniTokio {
    /// 상당히 비효율적인 방식의 executor이다.
    /// 왜냐하면 이는 단순 무식하게 instance를 꺼내와서 polling하고 상태 확인하고 ready면 끝내고 아니면 다시 queue에 넣는 방식으로
    /// 이는 cpu사용률을 증진시키기에 최적화된 구조이다.
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