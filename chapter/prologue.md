# 시작

## Rust 설치

rust는 rustup이라는 툴을 통해 버전 및 도구를 관리할 수 있다.

## rustup 설치 - Linux or MacOS

간단하게 스크립트를 통해 설치할 수 있다

```shell
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

맥의 경우 컴파일러나 링커를 찾을 수 없다면 아래 스크립트를 실행하여 툴체인을 설치하자
```shell
xcode-select --install
```

## rustup 설치 - Windows

https://www.rust-lang.org/tools/install

위 링크를 통해 installer를 받고 실행하자

## 설치 확인

정상적으로 설치가 됐으면, 버전 확인을 해보자

```shell
rustc --verison
```

형식은 아래와 같이 버전 숫자, 커밋 해쉬, 커밋 날짜로 출력된다

rustc x.y.z (abcabcabc yyy-mm-dd)

## 업데이트 및 삭제

업데이트
```shell
rustup update
```

삭제
```shell
rustup self uninstall
```

## Cargo 사용

Cargo는 화물이라는 뜻으로 rust 프로젝트를 하나의 화물로 취급해 관리한다고 보면 된다

즉 이를 풀어보면 rust에서 사용하는 의존성 라이브러리들을 관리하거나, 빌드 및 테스트, 프로젝트 생성 등의 코드외의 보일러 플레이트 적인 요소들을 관리한다

쉽게 빌드 및 패키지 매니지먼트라고 보면 된다

### Cargo로 프로젝트 관리

우선, 아래 명령어를 이용해 프로젝트를 생성하자
```shell
cargo new hello_cargo
```

그러면 .gitignore, src디렉토리, Cargo.toml이라는 파일이 생성된걸 볼 수 있다

이중 Cargo.toml이 있는데 이는 프로젝트 설정 파일로, rust의 매니페스트파일이라고 보면된다

( npm의 package.json, gradle의 build.gradle, go의 go.mod 등 과 비슷한 포지션 )

> Cargo.lock 파일은 협업시 의존성 라이브러리 충돌을 방지하기 위해 사용하는 파일로 낙관적락 version을 통해 관리한다

### Cargo를 이용한 빌드 및 실행

아래 명령어를 통해 빌드를 진행한다

```shell
cargo build
```

완료가 되면, target/debug 밑에 바이너리 파일이 생성되는걸 볼 수 있는데, 이것이 바로 빌드된 실행파일이다

만약 컴파일이 아닌 프로그램을 바로 실행하고 싶다면, 아래 명령어를 이욯하자

```shell
cargo run
```

실행조차 하지않고 빠르게 실행여부를 흝어보고 싶다면 아래 커맨드를 이용하자

```shell
cargo check
```