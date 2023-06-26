# Rust

### Cargo의 경로

- Window : %USERPROFILE%\.cargo\bin
- Unix : $HOME/.cargo/bin

`CARGO_HOME/bin` 경로는 캐시파일을 저장하고 있다.

## Window

Window에는 두 가지의 ABI가 있다.
첫 번째로는 Free Software재단(GNU)의 GCC Toolchain에서 사용하는 GNU ABI와 VS에서 사용하는 MSVC ABI가 있다.
> Application Binary Interface로 OS에서 프로그램 어떻게 처리할지에 대한 처리방식명세서라고 보면된다.

보편적으로는 Visual Studio의 msvc toolchain을 많이 쓴다. MS에서 지원해 윈도우 친화적 및 호환성이 좋다.

반면 Linux GNU Toolchain를 윈도우에서 사용할 수 있게 한 MinGW/MSYS2 toolchain도 있다
> MinGW/MSYS2의 경우 Cygwin기반의 도구 및 라이브러리 모음이다. Cygwin의 경우 윈도우에서 POSIX API 및 GNU Tool을 사용할 수 있게 한 프로그램이다
> 현대 윈도우에 와서는 Cygwin대신 WSL을 쓰면 되긴한다.

```shell
rustup set default-host x86_64-pc-windows-gnu

rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-pc-windows-gnu
rustup target add i686-pc-windows-msvc
rustup target add i686-pc-windows-gnu
```
위와 같이 toolchain을 선택할 수 있으며 rust에서 윈도우에게 총 4개의 toolchain을 제공한다.

## Rustup

Rustup은 toolchain multiplexer이다. python의 pyenv, node의 nvm과 비슷하다. 선택에 따라사 어떠한 toolchain을 사용할지 결정할 수 있기 때문이다.

### 채널

Rust는 stable, beta, nightly세 가지의 버전을 지원한다.
stable의 경우 6주마다, beta의 경우 다음 stable에 포함될 버전, nightly는 매일 밤 마다 이루어진다.

`rustup toolchain install nightly` nightly channel toolchain설치

### 프로필

- minimal의 경우 가능한한 적은 수의 컴포넌트가 포함된다 (rustc, rust-std, cargo)
- default의 경우 rust-docs, rustfmt, clippy등이 추가된다. ( 일반적으로 권장 됨 )
- complete의 경우 모든 component요소가 추가된다.

`rustup set profile [minimal|default|complete]`으로 프로필 설정 가능하다.

## 최신상태 유지

1. `rustup update` 업데이트
2. `rustup set auto-self-update [disable|enable|check-only]` 자동 업데이트 여부 check-only는 업데이트 있을 시 알림만 진행

[ Rust Toolchain 버전 확인 ]
> https://rust-lang.github.io/rustup-components-history/x86_64-apple-darwin.html