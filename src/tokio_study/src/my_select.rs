//! 여태까지 동시성 시스템을 제공하기 위해 Thread 생산했다
//! 그러나 select라는 새로운 방식을 도입해 동시성과 비동기 실행을 지원하자

use std::future::Future;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::net::TcpListener;
use tokio::sync::oneshot;

/// select는 여러개의 Channel중 먼저 Complete Channel의 값을 val로 반환하고 나머지 Channel의 Receiver를 Drop한다.
pub async fn execute_select() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        let _ = tx1.send("one");
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }
}

async fn some_operation() -> String {
    "some_operation".to_string()
}

/// Future는 poll을 할 때만 실행된다. 즉 Future를 Drop하면 실행하지 않고 삭제된다.
///
/// 이 내용을 이용해 Channel에서도 Receiver를 Drop하면 반대편인 Sender에게 notification이 전달되며 그 즉시 process를 abort한다.
///
///
pub async fn cancel_select() {
    let (mut tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        tokio::select! {
            val = some_operation() => {
                let _ = tx1.send(val);
            }
            _ = tx1.closed() => {
                println!("Closed")
            }
        }
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }
}

pub struct MySelect {
    pub rx1: oneshot::Receiver<&'static str>,
    pub rx2: oneshot::Receiver<&'static str>,
}

impl Future for MySelect {
    type Output = ();

    /// Select를 풀어헤치면 아래와 같다. 두 개의 Channel중 먼저 완료되는 Channel이 반환되며 당연히 남아있는 Future은 자연스럽게 Drop된다.
    ///
    /// 중요한 내용이 하나 더 있는데, Waker이다 정확한 시기에 wake를 해주지 않으면 무한 루프 혹은 CPU사이클 낭비로 이루어진다.
    ///
    /// 아래 케이스의 경우 wake작업을 channel에게 context를 넘겨주면서 맡기고 있다.
    ///
    /// 즉 channel이 pending이면 해당 future도 pending, ready면 ready가 되는 것 이다.
    ///
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
            println!("rx1 completed first with {:?}", val);
            return Poll::Ready(())
        }

        if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
            println!("rx2 completed first with {:?}", val);
            return Poll::Ready(())
        }

        Poll::Pending
    }
}

/// select의 Handler로 부터 return value를 받을 수 있다.
async fn return_value() {
    let out = tokio::select! {
        res = async {
            "res"
        } => {
            res
        }
        res = async {
            "res1"
        } => {
            res
        }
    };

    println!("{}", out);
}

/// Async Expression에서 Error를 Handler로 전파할 수 있으며 Handler에서는 select Caller로 Error를 전파한다.
async fn error() -> io::Result<()> {
    let listener = TcpListener::bind("localhost:3465").await?;

    tokio::select! {
        res = async {
            let (socket, _) = listener.accept().await?;

            Ok::<_, io::Error>(())
        } => {
            res?;
        }
    }

    Ok(())
}

/// Pattern에서 else를 사용할 수 있다.
async fn match_pattern() {
    tokio::select! {
        Some(v) = async {
            Some("some")
        } => {
            println!("Some : {}", v);
        }
        else => {
            println!("else");
        }
    }
}

/// select는 task와 다르게 소유권 이전이 필요없다. 빌림을 사용해서 제공해주면 된다.
/// 아래는 가변참조자인 push_str를 통해 두 개의 handler에서 값을 수정하는걸 볼 수 있다.
async fn borrowing() {
    let mut out = String::new();

    tokio::select! {
        _ = async {} => {
            out.push_str("1");
        }
        _ = async {} => {
            out.push_str("2");
        }
    }

    println!("{}", out);
}

/// Loop와도 사용할 수 있다.
async fn loop_test() {
    loop {
        let msg = tokio::select! {
            Some(msg) = async {
                Some("msg")
            } => {
                msg
            }
            else => {
                break;
            }
        };

        println!("{}", msg);
    }
}

/// 참조자에 await을 적용할 땐 Pin을 사용해야 한다.
///
/// Pin을 사용함으로써 해당 객체는 현재 메모리 영역에 고정되어 있으며, 절대 움직일 수 없다고 보장받을 수 있게된다.
///
/// 만약 Pin을 사용하지 않게되면 참조자의 원본 값이 다른 메모리 영역으로 이동해버릴 수 있고 이러면 댕글링 포인터가 되기에 문제가 된다.
async fn pin_test() {
    let operation = async {
        "operation"
    };
    tokio::pin!(operation);

    tokio::select! {
        val = &mut operation => return
    }
}