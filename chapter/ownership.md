# 소유권

소유권은 러스트에서 메모리를 관리하는 방식으로, 특정 메모리 영역에 대해 소유권을 지정하고 해당 소유자만이 그 영역을 사용할 수 있다

후술에서 설명하겠지만, 소유자가 아닌 대상이 해당 메모리에 접근하려면 빌림이라는 개념을 사용해야 한다

> 스택 vs 힙
> 
> 메모리에 올려둔 데이터를 관리하는 방식으로 스탭과 힙이 있다. 스택은 쉽게 자료구조를 떠올리면 되는데,
> 적재하려는 데이터를 FILO방식으로 쌓아올린 뒤 필요할 때 마다 컨텍스트에서 꺼내 사용하는 방식으로 동작하며
> 스택에 적재된 데이터는 컨텍스트의 스코프가 종료됨으로써 사라진다
> 
> 반면 힙은 힙영역이라는 특별한 공간에 데이터를 저장하고 이를 참조하는 방식으로 동작한다. 이는 스택과 달리 컨텍스트가 끝남에 따라
> 사라지는것이 아닌 CG, Free, Ownership등 언어마다 다양한 방식으로 이를 관리한다.

## 소유권 규칙

1. 각각의 값은 소유자가 있다
2. 값과 소유자는 1:1 관계이다
3. 소유자가 스코프 밖으로 벗어날 때 값은 버려진다

## String 타입

여타 다른 언어와 마찬가지로 `"Hello, World"` 쌍따옴표를 이용한 문자열은 리터럴로 불변성을 띈다. `( &str이 이와 같음 )`
이는 컴파일 시점에 고정된 크기임을 인지하여 Stack에 관리한다는 특징이 있다

반면 Rust에서도 String을 Heap영역에 두고 관리할 수 있는데, `String::from("Hello, World")`를 이용하면 Heap으로 관리할 수 있게 된다

## 메모리와 할당

Rust에서는 소유권을 잃은 값은 자동으로 해제되며, 스코프가 종료되기 이전에 소유권을 외부로 이동하지 않는이상 소유권은 자동으로 해제된다

```rust
{
    let s = String::from("hello");
}
```

위 코드에서 {}스코프가 끝나는 시점에 소유권은 사라지고 소유권이 사라짐에 따라 메모리도 해제된다

> C++에서 데이터의 사용이 끝나는 시점에 리소스를 해제하는 패턴을 Resource Acquisition Is Initialization (RAII)라고 한다

## 소유권 이동

```rust
let x = 5;
ley y = x;
```
5는 primitive type으로 고정적인 길이를 가지고있다. 따라서 이는 스택에 저장 되며, y는 x에 대해 깊은복사와 동일한 효과를 가진다.

```rust
let s1 = String::from("hello");
let s2 = s1;
```
s1은 Heap에 저장된 문자열로 s2 = s1구문으로 인해 타 언어는 얕은복사로 진행이 되는데, Rust에서는 소유권 이전이 된다.

즉 더 이상 s1변수에 대해 접근할 수 없어지게 되는 것 이다.

### clone

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```

clone을 통해 Heap영역까지 깊은복사를 진행했고, 이렇게 되면 소유권이전이 아닌 새로운 값이 생성되는 것이다.

## 소유권과 함수

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    
    let x = 5;
    makes_copy(x);
}
```

s의 경우 소유권이 이전되며, x의 경우 복사가 이루어진다.

## 반환값과 스코프

```rust
fn main() {
    let s1 = gives_ownership();
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}
```

gives_ownership함수에서 리턴한 문자열은 s1로 소유권이 이전된다.

## Copy, Drop 트레잇

스택에서 값을 복사할 수 있는 Copy트레잇이 있으며, 스코프가 종료되면 값을 버리는 Drop트레잇이 있다.

기본적으로 primitive type들은 Copy트레잇을 구현하고 있다 ( 정수, bool, 부동 소수점, char, (i32, i32)와 같은 튜플 )

## 참조자와 빌림

```rust
fn main() {
    let s1 = String::from("hello");
    take_parameter(s1);
    println!("{}", s1);
}

fn take_parameter(s: String) {
}
```

위 코드는 아쉽게도 에러가 발생한다. 이는 take_parameter함수가 s1의 소유권을 가져가기 때문이다.

이렇듯 함수로 넘겨준 파라미터에 대해 함수호출이 끝나더라도 사용하고 싶다면 Reference를 사용해야 한다.

### 참조자

```rust
fn main() {
    let s1 = String::from("hello");
    take_parameter(&s1);
    println!("{}", s1);
}

fn take_parameter(s: &String) {
}
```

단순하게 변수에 &키워드를 붙이면 참조자가되며 이는 해당 변수의 값이 아닌 주소를 넘겨준다.
> 포인터와 동일한 내용이며 위 예제에서는 s -> s1 -> "hello" 순으로 접근하고 있다.
> 
> 참조와 반대의 개념인 역참조도 있는데, 이는 *로 표현한다

이처럼 참조자를 만드는 행위를 빌림이라고 한다. 당연히 이 빌린값은 수정할 수 없으며 Only Read만 된다.

### 가변참조자

```rust
fn main() {
    let mut s = String::from("hello");
    
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

가변변수를 만들어 두고, mut 참조자를 만들어 이를 넘겨주면 된다.
이렇게 되면 빌려가는 쪽에서 해당 변수에 대해 접근해 Write작업이 가능해진다.

가변 참조자에는 제약사항이 2가지가 있다.

1. 가변참조자는 하나만 생성할 수 있다.
2. 가변참조자는 불변참조자와 함께 사용할 수 없다.
3. 참조자는 정의된 지점부터 마지막으로 사용된 지점까지만 유효하다.

1번 사항으로 인해 Race Condition으로부터 보다 안전해지며, 2번 사항으로 인해 변수의 불변성을 지킬 수 있다.

3번 사항의 예를 한번 보자

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
println!("{} and {}", r1, r2);

let r3 = &mut s;
```

r1과 r2에 대해 println에서 마지막으로 사용했으므로 3번사항에 의해 r1,r2는 만료된다.

따라서 2번 사항에 대해서도 만족하며 가변참조자를 사용할 수 있게 된다.

### 댕글링 참조자

댕글링 포인터(dangling pointer)란 어떤 메모리를 가리키고 있는 포인터가 있는 상황에서 해당 메모리가 해제되어 다른 데이터가 할당되어 있을 지 모르는 메모리를 참조하게 된 포인터이다.

```rust
fn main() {
    let dangling_pointer = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    
    &s
}
```

dangle함수에서 포인터변수 s를 반환해주지만 해당 함수의 스코프가 끝남으로써 s의 메모리는 해제된다.

## 슬라이스

컬렉션의 연속된 요소를 참조할 수 있게 함

```rust
let s = String::from("hello world");

let s1 = &s[0..5];
let s2 = &s[..5];
let s3= &s[..len];
```

슬라이스는 참조자라는 특성이 있는데, 이 때문에 보다 안전한 코드가 된다.

```rust
let s = "Hello, World!";
let s1 = &s[0..5];
```

s와 s1은 서로 같은 타입인 &str 불변참조자 문자열로, 동일한 형태이다.

```rust
let s0 = "Hello, World!Good"
```

만약 여기서 s0이 등장하고 s를 &s0[..13]로 설정하면 둘은 결과값이 같기에, s또한 하나의 리터럴로 볼 수 없다

컬렉션은 단순 문자열뿐 아닌 배열, 컬렉션 등 모두에게 해당한다

