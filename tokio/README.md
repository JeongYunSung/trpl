# Tokio

Rust를 기반으로 작성된 비동기 라이브러리로, 적은 스레드로 높은 동시성을 추구하는 논블로킹 I/O를 추구한다.
따라서 Thread를 많이 사용하는 Multi Thread환경 혹은 하나의 작업을 여러개의 작업으로 나누어 처리하는 Parallel 환경에서 쓰기엔 부적합하다.
또한 FFI를 지원하지 않는다.

## Tokio Runtime

`#[tokio::main]`매크로를 붙이면 해당 위치에 Tokio Runtime이 설정이 되는데, 이는 다음과 같은 역할이 있다

1. async function에 대해 block 처리
2. task에 대한 scheduling
3. thread pool 관리

## Tokio Thread

Tokio는 N:M Thread방식으로 동작한다. 즉 Tokio에서 만들어 내는 Thread는 기본적으로 Green Thread에 속한다.

```rust
let handle = tokio::spawn(async {
    "return value"
});
```
다음과 같이 Thread를 생성할 수 있는데, spawn을 하게되면 JoinHandle이라는 구조체를 반환하며, 이를 await시키면 Thread의 결과값이 반환된다.

JoinHandle의 경우 내부적으로 Future의 블록을 Task단위로 패키징시키는데, 이 때 이 Task가 Runtime에 의해 Schedule되어 실행된다.
Task는 매우 가벼우며 하나의 할당과 64바이트의 메모리만 사용하여 최대 수천개의 Task를 자유롭게 생성할 수 있다.

### 주의
1. Schedule의 방식이 Worker Steal로 진행되기에 하나의 Task가 여러 Thread를 오가며 실행될 수 있는데,
이러한 특성때문에 Future에서 사용하는 모든 개체들은 Send Trait을 구현해야 한다.
만약 구현되지 않았다면 Thread가 await 혹은 yield되기 이전에 스코프를 강제로 닫아 Drop를 유도하거나 명시적으로 Drop을해 Memory에서 release가 필요하다.

2. Task의 경우 한번 실행된 후 언제까지 lifetime이 유지될지 모르기에 Task에서 참조하고 있는 모든 값은 생에주기가 static lifetime으로 설정되어야 한다.
따라서 참조하고 있는 값이 `'static`이거나 혹은 Copy 또는 아예 `move`를 통해 소유권 이전을 시켜주어야 한다.

## Shared State

여러 Thread에서 하나의 자원에 대해 공유가 필요할 때 Mutex를 이용한다.

> std::Mutex vs tokio::Mutex
> 
> std의 Mutex는 동기식 Mutex이며 앞서 설명한 Tokio Thread의 1번 문제점을 가지고 있다. 
> 반면 tokio의 Mutex는 비동기식이며 await동작을 하더라도 lock을 유지시켜주며 동시에 unlock까지 정상적으로 진행해준다.
> 다만 이러한 점 때매 동기식 Mutex보다 리소스가 더 필요하다는 단점도 있다.

## Channel

`데이터를 공유하고 통신하지 말고, 통신을 통해 데이터를 공유해라`라는 말의 내용은 결국 Mutex를 이용하지말고 Channel을 이용해라 라는 의미이다.

Tokio에서 제공하는 채널의 종류는 크게 4가지로 다음과 같다.

1. mpsc : Multi Producer Single Consumer -> 여러개의 생산자와 하나의 소비자
2. oneshot : Single Producer Single Consumer -> 하나의 생산자와 하나의 소비자
3. broadcast : Multi Producer Multi Consumer -> 여러개의 생산자와 여러개의 소비자
4. watch : Single Producer Multi Consumer -> 하나의 생산자와 여러개의 소비자

Sender를 통해 데이터를 보내면 Receiver를 통해 데이터를 수신한다.

## I/O

Tokio의 I/O는 비동기적이며 AsyncRead와 AsyncWrite Trait을 구현한다. 예로 TcpStream, File, Stdout 등이 있다.

AsyncReadExt, AsyncWriteExt는 기존 Trait에 대해 확장한 버전으로 여러가지 편의 메서드를 제공한다.
> read()에서 Ok(0)가 반환된다면 Remote Socket즉 Stream이 닫혔다는 의미 (EOF)
> 
> TcpStream에서 Ok(0)을 준다면 꼭 return 등을 통해 끝내주어야 한다.
> 
> 만일 계속 read()를 할경우 값을 즉시 반환해주는데, 이러면 무의미한 CPU사이클만 늘어날 뿐 이다.

`AsyncReadExt.read()`를 통해 읽어온 데이터를 bytes[]에다 쓰며, `AsyncWriteExt.write()`를 통해 bytes[]에 내용을 Writer에 작성한다.

> write와 write_all의 차이는 단순 반환차이인거 같다. write로 썻을때도 40KB까지는 정상적으로 한번에 써지는건 확인했다. 

### split

`copy`의 경우 A socket에서 B socket으로 데이터를 복사하는데, 이렇게 되면 두 소켓에 대해 mutable이어야 하지만
기본적으로 TcpStream에서는 Read + Write가 합쳐진 하나의 Socket을 뱉게된다.
따라서 이럴때 Socket분리를 해야한다면 `split`을 이용하면 된다.

```rust
let socket = TcpStream::connect("127.0.0.1:8080").await?;
let (mut rd, mut wr) = io::split(socket);
```
> io::split vs TcpStream::split
> 
> io::split의 경우 Mutex와 Arc로 이루어져 있어 오버헤드가 있는 반면 
> TcpStream::split의 경우 단순 참조를 이용하기에 zero-cost이며 split()을 한 곳과 동일한 task에서만 사용할 수 있다. (참조 특성)

### Buffer

BufWriter를 통해 Buffer에다 Write를 할 수 있으며, 해당 Buffer로부터 데이터를 Read할 수 도 있다.
Write의 경우 항상 flush를 해주어야 된다.

`BufWriter<TcpStream>`로 활용

### Cursor

Cursor를 통해 특정 Byte Array에 position을 조정할 수 있다.

## Async

### Future

std library로 비동기의 결과값을 가져오기 위해 제공되는 Trait이다.
Future은 poll함수를 통해 계산하고 값을 가져올 수 있으며, 이 방식을 await키워드를 통해 묵시적으로 진행할 수도 있다.

```rust
fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    if Instant::now() >= self.when {
        println!("Future Doen");
        Poll::Ready("done")
    } else {
        println!("Future Pending");
        Poll::Pending
    }
}
```
아직 계산중이라면 Pending, 계산이 완료됐다면 Ready를 반환해주면 된다.

### Notify

Tokio에서 제공하는 개체로, 특정 notify를 block시켜 재울 수 있으며, 다른 쪽에서 nofity를줘 wake할 수 있다.

```rust
thread::spawn(move || {
    let now = Instant::now();

    if now < when {
        thread::sleep(when - now);
    }

    notify2.notify_one();
});

notify.notified().await;
```

### Executors

Executors는 Future의 poll을 schedule하고 가장 외곽에 있는 Future에 대한 실행을 담당한다.
즉 Tokio Runtime의 주 역할중 하나가 바로 Executors인 것 이다.

Executors는 여러가지 방식으로 구현될 수 있는데, 대표적으로 다음과 같다.

1. Queue를 활용해 Future를 Queue에 넣고 하나씩 가져오면서 poll 이 후 pending이면 다시 Queue에 삽입
2. Channel과 Waker를 활용한 방식으로 Thread가 Spawn할 때 Channel Send하고 Receiver를 받은 Executors에서는 polling을 수행
이 후 해당 Future에서 Waker를 통해 다시 Channel에 Send그리고 이를 다시 receive해서 polling이를 반복

전자의 경우 CPU사이클이 추가적으로 소모되며 후자의 경우 필요한 경우에만 poll을 수행하기에 훨씬 효과적이다.

> std::Channel의 경우 sync가 구현이 되어있지 않아 crossbeam의 Channel을 사용
> 
> Waker Hooker는 futures의 ArcWake를 사용, Hooking직접 하려면 RawWakerVTable을 이용해야 함

### Waker
매 번 데이터를 poll해와 state를 비교하는 것은 비효율적이다. 
이를 개선하기 위해 나온 개념으로 waker를 통해 wake를 하면, polling이 준비됐다는 의미로 executor에서 다시 해당 future을 poll한다.

[ 활용 ]
1. Waker는 Task내의 Context내에 존재하며, 이를 활용할 수 있다.
2. Waker.wake_by_ref는 효과적이고 암묵적이게 소유한 Waker에 대해서 wake처리를 한다.
3. Waker.wake를 통해 Waker를 소비하면서 wake처리를 강제적으로 할 수 있다.
4. 하나의 Future가 여러 Thread를 오가며 polling을 할 수 있는데, 이 때는 가장 마지막에 있는 Task가 가지고있는 Waker를 통해서 wake해주어야 한다.

## Select

비동기를 다룰때 Thread Spawn하는 방식과 여러 Async를 한번에 실행시켜 이 중 패턴에 통과한 Future에 대해서만 처리하는 Select방식이 있다.

```rust
tokio::select! {
    _ = async {} => {
        out.push_str("1");
    }
    _ = async {} => {
        out.push_str("2");
    }
}
```
위의 코드는 머가 먼저 실행될지 모른다. 허나 확실한건 하나의 Future만이 실행되며 실행중이던 Future는 Scope가 종료되면서 Drop된다.
> 50%확률로 1, 50%확률로 2가 출력되는거 같음

### Poll로 표현
```rust
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
```
위 poll함수에서 랜덤요소가 좀더 추가된게 결국 Select와 같다고 보면 된다.

둘 중 어떤 Future가 Ready가 되느냐에 따라 rx1가 출력될지, rx2가 출력될지 결정되기 때문이다.

Future의 poll을 구현할 땐 중요한점이 Waker의 wake를 해야하는데, 이 부분에 대해선 channel의 poll에게 context를 위임하여 처리시키고 있다.

### 문법

`<pattern> = <async expression> => <handler>`로 select는 표현이 된다.

1. pattern은 async expression의 reuslt를 handler가 사용할 수 있게 하는 변수로 쓰이거나 혹은 else로 쓰인다.
2. async expression은 말그대로 비동기 함수에 대해서 표현한다.
3. handler는 해당 비동기 함수의 처리가 ready인 경우 진행되는 process이다.
4. select!는 task와 다르게 borrow를 사용한다.
5. 만약 async expression에서 참조자를 기준으로 await을 할경우 pin을 꼭 적용해주어야 한다
> Pin을 적용해주지 않으면 개체가 소멸되거나 다른 메모리로 이전됐을 경우 참조자는 댕글링 포인터가 되므로 문제가 발생

```rust
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
```
Return Value로서의 활용

```rust
let listener = TcpListener::bind("localhost:3465").await?;

tokio::select! {
    res = async {
        let (socket, _) = listener.accept().await?;

        Ok::<_, io::Error>(())
    } => {
        res?;
    }
}
```
에러 전파

```rust
let mut out = String::new();

tokio::select! {
    _ = async {} => {
        out.push_str("1");
    }
    _ = async {} => {
        out.push_str("2");
    }
}
```
mutable reference사용