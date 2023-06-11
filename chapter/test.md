# 테스트

테스트는 크게 필요한 데이터 설정(given), 코드 실행(when), 의도한 결과 확인(then)으로 구성된다.

## 테스트 작성

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}
```
단순하게 test 어노테이션을 붙여주고 cargo test를 실행해주면 된다.

```shell
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
출력결과는 위와 같은데 각각 아래와 같은 의미를 가진다.

- passed: 테스트가 성공적으로 통과했다.
- failed: 테스트가 실패했다.
- ignored: 테스트를 무시했다.
- measured: 성능 측정 벤치마크

### 다양한 테스트 매크로

- assert! : 조건이 true인지 확인한다.
- assert_eq! : 두 값이 같은지 확인한다.
- assert_ne! : 두 값이 다른지 확인한다.

### 다양한 어노테이션

- #[test] : Test 코드라고 알림
- #[should_panic] : panic이 발생했는지를 테스트
- #[ignore] : 테스트를 무시

## 테스트 실행 제어

cargo test는 test로 전달할 인수와 테스트 바이너리에 전달할 인수를 구분짓기위해 -- 구분자를 쓴다.
```shell
cargo test args -- --test-threads=1
```
방식은 위와같다. 위 케이스는 테스트를 실행할 스레드의 개수를 1개로 제한한다 ( 직렬 실행 )

```shell
cargo test {test_name}
```
위 실행 결과는 SQL로치면 whete test_name like '{test_name}%' 이런 느낌이다.
즉 해당 test_name과 일치하거나 start with하는 테스트를 실행한다.

```shell
cargo test --test 파일명
```
위 커맨드를 통해 특정 파일에 대해서만 테스트를 진행할 수도 있다.

## 테스트 조직화

유닛테스트 : 하나의 모듈, 기능과 같이 국소적인 부분에 대한 테스트
통합테스트 : 외부 입장에서 라이브러리를 사용하듯 전체적인 부분에 대한 테스트

### 유닛테스트

유닛테스트는 cfg(test)를 어노테이션 하는게 일반적인 관례이다. ( cfg는 configuration의 약자 )

```rust
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn internal_test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
```
위 같이 테스트할 수 있는데 일반적으로 비공개 함수 테스트시 intenal을 붙인다.

### 통합테스트

#### 작성

통합테스트는 아래와 같은 순서로 작성할 수 있다.

1. 패키지밑에 tests라는 폴더를 만든다.
2. intergration_test와 같은 이름을 가진 러스트파일을 생성한다.
3. 해당 러스트파일 안에 테스트를 작성한다.

#### 특징

통합테스트의 특징은 각 rust파일마다 새로운 Crate라는 것 이다. 따라서 아래와 같은 특징이 생기는데,

1. root crate의 라이브러리 crate를 참조할 수 있다.
2. A라는 test파일에서 B라는 test파일의 scope을 참조할 수 없다.
3. tests 폴더 안에 모듈을 만들어서 여러 test crate이 공유할 수 있다.

위 특징을 잘 활용하면 test를 여러 파일로 분리 및 모듈화를 할 수 있다.