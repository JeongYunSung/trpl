# Syntax Extension

## Source Analysis

### Tokenization

Rust컴파일러의 첫 번쨰 단계는 토큰화이다. raw text의 sequence를 token단위로 변환 시키는 구간이다.
이 때 Rust는 4가지의 Type으로 토큰을 분리한다

1. 식별자 : foo, name, jys, self, ... 와 같은 변수명, 함수명 등
2. 리터럴 : 42, 0x16, 72u32, 0______0, "const", ... 와 같은 상수 타입
3. 키워드 : _, fn, self, match, yield, macro, await, async, ... 와 같은 예약어
4. 기호 : [], :, ::, ?, ~, @, ... 배열, 스코프, Path, 메서드 호출 등의 기호

```c
#define SUB int
#define BEGIN {
#define END }

SUB main() BEGIN
    printf("Hello, World!\n");
END
```
C에서는 define을 통해 매크로를 계층적으로 표현할 수 있다.

### Parsing

TokenStream이 AST로 변환되며, 실제 메모리에서 프로그램의 구문구조를 구축하는 작업이 포함된다.

### Token Tree(tt)

토큰 트리는 토큰과 AST사이에 있다. 토큰은 Tree의 Root가 될 수도 Leaf가 될 수도 Branch가 될 수도 있다. ( 일반적으로 대부분 Leaf )
그러나 그룹화 토큰 (...), [...], {...}의 경우는 절대 Leaf Token이 될 수 없다.

## AST Macro

