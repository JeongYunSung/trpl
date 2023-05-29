# 설치

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

