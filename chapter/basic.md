# 프로그래밍 개념

## 변수

### 불변수와 가변수

Rust의 기본 변수는 불변이다 요새 많은 언어들이 기본 변수를 불변으로 두는 추세인데, 이는 동시성 문제로부터 안전하다는 장점이 있다

```rust
let x = 5;
x = 3;
```

당연한 소리지만 위 코드는 정상적으로 작동하지 않는다

만약 값을 변경하고 싶다면 `mut` 키워드를 사용하면 된다

```rust
let mut x = 5;
x = 10;
```

정상적으로 값이 변경되는 걸 알 수 있다

> mut은 mutable의 약자


### 상수

상수는 항상 불변인 값을 뜻하며 런타임중에 임의적으로 값이 할당되는 것이 아닌, 컴파일 시점에 값이 할당된다.

```rust
const SECOND = 60;
const MINUTE = SECOND * 60;
```

### 섀도잉

쉽게 말해 변수를 똑같은 이름으로 재정의 하는 것인데, 이렇게 되면 기존 변수가 재정의된 변수에 의해 가려진다고 해서 섀도잉 이라고 한다.

```rust
let x = 5;
println!("x: {}", x); // x: 5
let x = x + 5;
println!("x: {}", x); // x: 10
```

섀도잉의 무분별한 남용은 의도치 않은 변수를 참조할 수 있어 주의가 필요하다

## 데이터 타입

러스트는 정적언어로 컴파일 시점에 Lexical분석이 처리 되며, 이 시점에 변수의 타입 스코프 등이 결정된다.

따라서 컴파일 시점에 타입을 추론할 수 있게, 개발자는 상세하게 타입을 지정해줘야 한다. 다만 Optimizing에 의해 어느정도 까지는 자동추론을 해주고 있다.

### 스칼라 타입

단순히 값을 나타내는 타입이며 러스트에서는 정수, 부동소수점 숫자, Boolean, 문자에 대해 지원하고 있다.

1. 정수형

| 길이     | 부호 O | 부호 X |
|--------|------|------|
| 8bit   | i8   | u8   |
| 16bit  | i16  | u16  |
| 32bit  | i32  | u32  |
| 64bit  | i64  | u64  |
| 128bit | i128 | u128 |

2. 숫자 리터럴

| 숫자 리터럴        | 예           |
|---------------|-------------|
| Decimal       | 98_222      |
| Hex           | 0xff        |
| Octal         | 0o77        |
| Binary        | 0b1111_0000 |
| Byte(u8 only) | b'A'        |

### 복합 타입

Rust에서는 복합타입으로 튜플과 배열을 지원한다

1. 튜플

튜플은 아래와 같이 다양한 타입의 값을 하나의 변수로 묶을 수 있다 다만 고정적인 길이를 가지고 있다는 점을 알아두자

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

튜플은 아래와 같이 구조분해도 가능하다

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

2. 배열

배열은 튜플과 달리 []로 구문을 시작하며, 마찬가지로 고정적인 길이를 가지고 있다

러스트 표준라이브러리에는 벡터가 있는데, 이를 사용하면 가변적인 길이를 가진 컬렉션을 만들 수 있다

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

배열의 타입 및 길이를 정할 수 있다

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

## 함수

함수는 아래와 같이 선언할 수 있다
```rust
fn main() {
    println!("Hello, world!");
}
```

특히 main함수의 경우 프로그램의 시작지점이라고 할 수 있다

### 시그니쳐

함수의 이름(파라미터) -> 리턴타입 으로 구성된다

```rust
fn function(args: i32) -> i64 {
    100
}
```

### 구문과 표현식

여타 다른 프로그래밍언어와 마찬가지로 러스트는 크게 구문과 표현식으로 나뉜다

구문식은 특정 동작을 수행한 뒤 아무런 값을 반환하지 않으며 표현식은 값을 계산하고 반환한다는 특징이 있다

```rust
fn main() {
    let x = 5; // 구문

    let y = {
        let x = 3; // 구문
        x + 1 // 표현식
    };

    println!("The value of y is: {}", y);
}
```

특정 연산을 하거나, 스코프를 지정하는 블록또한 하나의 표현식이 될 수 있다

위 코드블록 중 y식을 잘보면 마지막 x + 1뒤에 세미콜론이 없는걸 볼 수 있는데, 세미콜론을 붙이게되면 이는 식으로 간주하여 반환값으로 사용할 수 없다 ( 블록의 마지막에서만 해당 )

반환은 표현식만을 반환할 수 있기 때문이다

## 제어 흐름문

### if ~ else if ~ else

일반적인 사용예

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

if문은 표현식으로 이를 활용한 방법

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

## 반복문

### loop

loop는 흔히 말하는 무한루프와 같은 개념으로 개발자가 명시적으로 멈추지 않는 이상 계속 반복한다

아래는 중첩 loop구조로 label을 주어 특정 loop에 대해서 제어를 할 수 있다
```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

loop에서 break를 사용하면 표현식으로서 활용할 수 있다

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

### while

조건문이 true인동안 반복한다

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

### for

Rust에서 제공하는 Range를 활용해서 하면 편하다

Range는 kotlin에서 until, n..m과 같은 개념이라고 보면된다

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```