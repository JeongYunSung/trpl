# 열거형

하나의 타입이 가질 수 있는 variant들을 열거함으로서 타입을 정의한다.

## 정의

```rust
    enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

V4, V6이라는 두개의 variant가 있으며 각각 파라미터를 받아 위와같이 처리할 수 있다.

```rust
struct Ipv4Addr {
    // --생략--
}

struct Ipv6Addr {
    // --생략--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
구조체, 이름이 있는 타입, 복합타입등을 받을 수도 있다.

```rust
impl Message {
    fn call(&self) {
        // 메서드 본문이 여기 정의될 것입니다
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```
뿐만아니라 위와같이 메서드또한 정의할 수 있다.

## Option

러스트에는 실제 null이 있지 않으나, 값이 없음을 표현할 수 있는 Option이라는 열거형이 있다.

```rust
enum Option<T> {
    None,
    Some(T),
}
```
Option은 Rust에서 기본적으로 import하는 preludus목록에 포함되어 있다.
> preludus : pre ~ 이전의, ludus 연주 즉 연주이전에 진행하는 전주곡이라고 볼 수 있다. 
> Rust에 이를 기본적으로 탑재되는 라이브러리라고 보면 된다.
 
## Match

매치는 패턴매칭을 통해 값을 도출해 낸다ㅏ.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    TEST,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        _ => 10,
    }
}
```
match키워드 뒤에 표현식이 오며, 각 arm마다 패턴과 코드로 분리되어 있다.
해당하는 값이 없을 시 _를 사용할 수 있다. ( 포괄 패턴 )

### Option<T>을 활용

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```
이런식으로 Option의 variant를 활용할 수 있다.

## if let

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
} else {
    count += 1;
}
```
단 한건에 대해서만 match를 사용할 경우 if let을 사용할 수 있다. 물론 else도 사용할 수 있다.
