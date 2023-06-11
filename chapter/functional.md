# 함수형 언어

어떠한 언어적 특징을 결정짓는데는 First Class Citizen이라는 개념이 있다. 이는 다음과 같은데

1. Class를 파라미터로 넘길 수 있다.
2. Class를 데이터나 자료구조 등에 담을 수 있다.
3. Class를 Return 할 수 있다.

Rust에서는 이 Class에 대해 Function을 지원하고 있어, 우리가 함수형 프로그래밍할 수 있게 해준다.

## 클로저

함수나 메서드내에 지역변수를 선언하게 되면, 해당 변수는 Stack에 올라가게 되고 자연스럽게 Stack의 Context가 끝나면서 이 변수는 소멸하게 되는데,
Closer는 해당 변수에 대해 독립적인 Context를 만들어, 해당 변수를 외부에서 사용할 수 있게 해준다. 즉 변수의 Scope를 확장시켜주는 것 이 바로 클로저이다.
> Closer vs Heap에 대해서 고민할 수 있을거 같다. 결국 둘은 Stack범위 밖에서 해당 자원에 접근할 수 있게 해주는 기술이기 때문에 헷갈릴 수 있을거라고 생각한다.
> 따라서 이 부분에 대해서 정리를 하자면 Closer는 보다 행위에 가깝고 Heap은 데이터에 가까운데, 먼저 Heap의 경우 데이터를 저장할 수 있는 성질이라면,
> Closer는 해당 데이터의 Scope을 넓혀 주는 성질을 가지며 이 때 사용하는 저장방식이 Heap인 것 이다.
> 
> 왜 그렇다면 Closer는 행위라고 말하는걸까 ? 이 부분은 함수형 프로그래밍으로 넘어오면서 부터 개념적인 성질을 가지게 되는데,
> First Class Citizen에 의해 함수를 인자로 넘길 시 함수가 들고 있는 스코프가 외부 스코프에서도 유효해야 한다.
> 즉 함수를 구성할 당시의 데이터를 그대로 보존해야 실행할 때 정상적인 결과를 보장받을 수 있다는 것 이다.
> 
> 따라서 함수를 호출했을 때의 함수 내부에서 사용하는 데이터를 실행하는 곳에서도 유지시켜야 하며 이를 특별히 Closer라고 부르는 것 이다.

```rust
fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
    user_preference.unwrap_or_else(|| self.most_stocked())
}
```
unwrap_or_else는 내부적으로 딱 한번만 실행시켜주는 (FnOnce() -> T)타입의 함수를 파라미터로 받는다.
그리고 이렇게 클로저를 사용할 떄 Rust에서는 || 연산자를 사용하며 이 때 매개변수가 있다면 |e: i32|와 같이 사용할 수도 있다.

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```
위를 통해 함수의 선언과 클로저의 선언 차이점을 확인해보자 ( 나는 단순 파라미터로 넘기는 클로저에 대해서는 익명함수에 더 가깝다고 생각한다 )

### 데이터 캡처 및 소유권 이동

```rust
fn main() {
    let list = vec![1, 2, 3];

    let only_borrows = || println!("From closure: {:?}", list);
}
```
불변변수 일 때는 포인터를 캡처해서 넘겨준다. ( 불변 참조 )

```rust
fn main() {
    let mut list = vec![1, 2, 3];

    let mut borrows_mutably = || list.push(7);
}
```
가변변수 일 때는 가변 참조로 넘겨준다.

```rust
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
```
만약 소유권 이전이 필요하다면 move키워드를 사용하면 된다.

### 클로저의 종류

1. FnOnce : 한 번만 호출되는 클로저에 적용되며, 캡처된 값을 클로저 본문 밖으로 이전 시킴
   * 본문 밖으로 이동시킬 경우 이 후 다시 실행할 때 소유권 문제를 겪을 수 있음
2. FnMut : 본문 밖으로 이전시키지는 않으나, 값을 변경할 수 있으며 한번 이상 호출 됨
    * FnMut내에서 데이터를 외부로 이동 시킬 시 FnOnce를 사용해야하고 그럼 1번의 문제에 봉착할 수 있음
3. Fn : 본문 밖으로 이동시키지도, 변경하지도 않으며 아무런 값도 캡처하지 않음 ( 동시성에서 중요 )

## Iterator

반복자는 컬렉션의 내부를 순회하는 Trait이다.

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```
Iterator Trait은 위와 같이 생겼다.
> Iterator의 next는 소비 어댑터(consuming adaptor)라고 하는데, 호출하면 반복자를 사용해버리기에 그렇다.
> 
> 예로 sum메서드의 경우 반복자의 소유권을 가져온다음 반복적으로 next를 호출해 순회하면서 반복자를 소비한 다음 해당 결과들을 합산한다.

## 다른 반복자를 생성하는 메서드

map의 경우 새로운 반복자를 만들어 낸다.

```rust
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);
```
위 코드는 실패하는데, 반복자를 만들어 낸 뒤 바로 소비해야 하기때문이다. 이 때는 `collect()`를 뒤에서 호출해주면 된다.

## 성능 비교: 루프 vs 반복자

러스트에는 비용없는 추상화(zero-cost abstraction)이라는 개념이 있는데, 이는 코드가 고수준으로 추상화 되어있더라도 컴파일 과정중 저 수준으로 변환이 되어 런타임 오버헤드가 없다는 것을 의미한다.

Iterator는 이런 zero-cost abstraction에 해당하며, 이에 엄청난 성능을 보인다 실제 벤치마킹한 결과를 보면 아래와 같이 Iterator가 좀 더 우세한걸 알 수 있다.

```shell
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```