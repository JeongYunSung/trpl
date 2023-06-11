# 에러

러스트 크게 복구 가능한 에러와 복구 불가능한 에러 2가지 범주로 묶는다.

## 복구 불가능한 에러

1. 배열의 크기를 넘어선 인덱스 접근과 같이 무조건 실패 및 복구를 해도 의미가 없는 에러를 복구 불가능한 에러라고 한다.
2. 위와 같이 의도적인 에러 및 panic!매크로를 이용하면 복구 불가능한 에러를 노출시킬 수 있다.
3. 패닉 메시지의 경우 메시지 출력 및 프로그램을 되감고, 스택 청소와 종료를 진행한다.
> 패닉 발생시 프로그램은 되감기를 진행하는데, 이 과정을 통해 메모리를 해제한다. 다만 이 과정이 많은 비용이 들어 이 옵션을 끌 수 있는데,
>  
> Cargo.toml파일 내에 `[profile.relase] panic='abort'`를 기입해주면 된다.
> 
> 다만 이렇게 할 경우 프로그램에서 메모리를 해제한게 아니므로 OS가 이를 청소해주서야 한다.

### backtrace

자바에서의 strack trace와 상응하는 개념으로, 패닉이 발생한 부분을 추적하는 기능이다.

기본적으로 cargo build, run을 --release옵션없이 실행하면 자동으로 backtrace기능이 활성화 되어 스택이 출력이된다.

## 복구 가능한 에러

파일이 없을 경우 실패가 아닌 새롭게 만드는 방식으로 복구를 할 수도 있다.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Result열거형을 통해 성공결과 혹은 에러를 받아 복구를 할 수 있다.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
```

error의 종류는 io:Error에서 제공하는 ErrorKind구조체를 통해 알 수 있다.
위 예시는 NotFound일 경우 파일을 생성하도록 하는 내용이다.

### 유용한 메서드

1. unwrap - Result가 Err일 시 panic을 낸다.
2. expcet - Err일시 지정한 메시지와 함께 panic을 낸다.

### 에러 전파

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```
1. ? 연산자를 통해 Err가 발생 시 해당 에러를 return시켜준다. 즉 return Err(e)와 동일한 효과를 가짐
2. ? 연산자는 From Trait에 정의되어 있는 from함수를 호출하여 x -> y타입으로 변경시켜 줌
3. ? 연산자는 Result, Option, FromResidual과 같은 특정한 반환타입에 대해서만 사용할 수 있음

> ? 연산자를 활용하여 에러전파에 대한 보일러플레이트를 최소화하자.

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```
next()메서드는 Option을 뱉는데, 이 때 Some이아닌 None을 뱉게되면 ? 연산자에 의해 바로 Option.None을 반환하게 된다. ( 즉시 종료 )

## Panic vs Result

1. 실패할지도 모르는 함수를 정의할 땐 기본적으로 Result를 사용해라
2. 유효하지 않은 타입, 이미 해제된 메모리에 접근, 유효하지않은 범위 등과 같이 의도치 않은 상황에 대해서 panic을 사용해라
3. 사실 이미 Rust에서는 소유권이라던가, 타입에 대해서 사전에 panic을 발생시키고 있다. 따라서 우리가 panic을 사용할일은 많지 않을 것 이다.

## 커스텀 타입

