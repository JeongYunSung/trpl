# 동시성

## 스레드

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
}
```
thread::spawn을 통해 생성할 수 있으며, join을 통해 blocking을 걸 수 있다.

### move 클로저

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```
위 코드에서 handle의 라이프타임이 main에 라이프타임보다 짧은거라는 보장이 없음으로 v의 소유권을 이전시켜주어야 한다.

이 때 클로저앞에 move키워드를 붙여 이행할 수 있음

## 메시지 패싱 ( 채널 )

Golang의 Concurreny 슬로건 `메모리를 공유하여 통신하지 마세요; 그 대신, 통신하여 메모리를 공유하세요.`

러스트는 메모리 동시성을 보장하기 위해 채널이라는 구현체를 제공한다.

채널은 송신자(transmitter)와 수신자(receiver)로 나뉘는데, 송신자가 강의 상류에 속하고 수신자는 하류에 속한다.
송신자가 버려지거나 종료되면 해당 채널은 닫힌 상태가 된다.

러스트의 채널은 송신과 수신의 관계가 N:1이다. 여러개의 강줄기가 결국 하나의 큰 강으로 합쳐지는 것과 같다. ( like 한강 )

```rust
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let val = String::from("hi");
    thread::sleep(Duration::from_secs(1));
    tx.send(val).unwrap();
});

let received = rx.recv().unwrap();
println!("Got: {}", received);
```
1. mpsc(multi producer single consumer)를 통해 채널을 만들고 send와 recv로 통신할 수 있다.
2. 당연히 recv동안은 blocking이 되며 try_recv를 이용할 시 값이 있으면 반환 없으면 Err를 주게 할 수도 있다.
3. 소유권은 채널과 함께 사용될 수 없으며, 당연히 채널로 보내진 데이터는 소유권또한 이전된다.
4. 채널의 생산자는 여러명이 될 수 있는데 이 때는 `let tx1 = tx.clone()`과 같이 clone을 사용하자

## 메모리 공유 상태 동시성

공유메모리는 사실 소유권이 복수인것돠 다름없다. Golang의 `슬로건중 데이터를 공유한뒤 통신`해라 부분이 공유상태 동시성을 말하는 것 이다.

### 뮤텍스

뮤텍스는 상호배제 즉 A라는 사건은 동시에 일어날 수 없다라는 가정하에 이루어진다.

뮤텍스는 락이라는 자물쇠와 열쇠시스템과 동일하다. ( 키가 있어야 열 수 있으며 사용이 끝나면 키를 반납 )

```rust
let m = Mutex::new(5);

{
    let mut num = m.lock().unwrap();
    *num = 6;
}

println!("m = {:?}", m);
```
Mutex는 위와같이 만들 수 있으며, lock()시 MutexGuard라는 객체를 반환하는데,
이는 스마트 포인터로 Deref시 내부 데이터를 가리키도록 및 Drop시 unlock할 수 있게 설정되어있다.

### Arc

Arc는 Rc와 동일한 참조 카운팅으로, Thread-safe한 Rc라고 보면 된다.

## Send, Sync Trait

### Send

Send Trait을 사용하여 소유권 이전을 허용할 수 있다. 즉 타입의 소유권이 스레드 사이에 이전될 수 있음을 나타내며
Rc, RefCell의 경우 Send Trait을 구현하지 않았다. ( 원시 포인터를 제외하곤 거의 모든 기초 타입이 Send이다 )

### Sync

Sync가 구현된 타입이 여러 스레드로부터 안전하게 참조 가능함을 나타내며, &T를 Send처럼 참조자가 안전하게 보내질 수 있다면 T는 Sync하다.
기초 타입들은 Sync하며, Sync타입으로만 구성된 타입또한 Sync하다.

( 불변참조를 통해 수정할 수 있으므로 동시에 액세스 하는것이 안전하지 않은 Cell이 Sync하지 않는 좋은 예시이다 )

## Async/Await

Async로 비동기 함수, Await으로 해당 함수에 대해 Blocking을 할 수 있음