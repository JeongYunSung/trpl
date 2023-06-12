# Cargo 심화

## 프로필 설정

빌드 최적화를 위해 Cargo에서는 프로필을 설정할 수 있다. 기본적으로 제공되는 프로필은 dev, release로 아래와 같이 설정되 어있다.

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```
opt-level은 optimizing level로 최적화 레벨을 뜻하는데, 숫자가 높을수록 최적화를 많이하며 빌드시간이 오래걸린다.

## Crates.io에 배포

crates.io의 registry에 작업한 패키지를 publish할 수 있다.

### 문서화

`///`와 Markdown을 이용해 문서를 작업하면 HTML문서로 뽑아준다.

```shell
cargo doc --open
```
문서를 열고 싶다면 위 커맨드를 이용하자

자주 사용되는 절은 아래와 같다.

1. Panics : 문서화된 함수가 panic을 일으킬 수 있는 시나리오로, 함수를 사용하는 쪽에서 자신의 코드를 작성할 때 참고할 수 있다.
2. Errors : 해당 함수가 Result를 반환할 때, 에러의 종류, 조건 등을 설명해준다.
3. Safety : 함수가 호출하기 전에 unsafe한 경우라면 이 부분에 대해 불변성, 왜 안전하지 않은지에 대한 설명을 함

```rust
/// 두 개의 숫자를 비교합니다.
///
/// # Examples
///
/// ```
/// use crate::trpl::add_one;
///
/// let arg = 5;
/// let answer = add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```
위와같이 문서화를 하면 cargo test도중 해당 마크다운을 읽어들인 뒤 해당 코드에 대한 테스트를 진행해준다.

#### 도입부

`//!`를 사용하면, 아이템을 문서화하는 것이 아닌 해당 주석을 포함하는 아이템들을 문서화한다. 가령
```rust
//! # TPRL
//!
//! The Ruet Programming Language에 대한 설명이다.
```
lib.rs나 모듈내 최상단에 위와같이 주석을 달아주면 된다. ( 관례상 src/lib.rs에 작성을 많이 함 )

### pub use로 공개 API 내보내기

```rust
pub use self::kinds::PrimaryColor;

pub mod kinds {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
}
```
pub use를 통해 re export를 할 수 있다. 그러면 문서에는 해당 PrimaryColor가 Re export됐다고 설명이 나온다.

## Workspace 사용

Cargo의 패키지에는 여러 패키지가 들어갈 수 있는데, 이러한 구조를 위해 패키지들을 묶는 패키지 즉 워크스페이스라는 개념이 탄생했다.

```toml
[workspace]

members = [
    "adder",
    "add_one",
    "add_two"
]
```
workspace 디렉토리 밑 Cargo.toml에 위와같이 [workspace] 기재 및 member들의 경로를 기재해주면 된다.

이 때 각 member는 하나의 Crate를 의미하며 바이너리 크레이트일 수 도있고, 라이브러리 크레이트 일 수 도있다.

```toml
[[package]]
name = "adder"
version = "0.1.0"
dependencies = [
 "add_one",
 "add_two",
]
```
또한 workspace를 만들게 되면 Cargo.lock파일이 각 Crate가 아닌 Workspace에 생기게 되며, 이를 통해 모든 Crate에서 dependency의 version을 동일하게 사용할 수 있게 해준다.

### 테스트

workspace에서 `cargo test`를 하게되면, 그 밑에 있는 Crate의 모든 test코드가 실행된다.

만약 특정한 Crate에 대해서 테스트를 실행시키고 싶다면, `cargo test -p adder`와 같이 `-p` 옵션을 사용하면 된다.

## 설치

Crate의 설치는 바이너리 크레이트에 대해서만 가능하다. 즉 실행가능한 파일이어야 한다는 소리이다.

`cargo install crate` 이 커맨드를 통해 crates.io에서 crate를 설치할 수 있다.

일반적인 설치 경로는 $HOME/.cargo/bin 밑에 설치되게 된다.