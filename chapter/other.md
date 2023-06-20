# unsafe

unsafe rust 즉 메모리 안전 보장이 적용되지 않은 rust를 일컫는다.

unsafe 블록의 리턴값은 Copy Trait을 구현한 객체의 경우 값복사, 구현하지 않은경우 unsafe블록 내에서만 사용해야 한다.

크게 5가지의 기능을 지원한다.

1. 원시 포인터 역참조
2. 안전하지 않은 함수 혹은 메서드 호출
3. 가변 정적 변수에 접근하기 및 수정
4. 안전하지 않은 트레잇 구현
5. union의 필드 접근

## 원시 포인터 역참조

raw pointer는 말 그대로 포인터를 의미한다. 즉 실제 Rust에서 참조하는 메모리의 주소이다.
우리가 일반적으로 사용하는 참조자(&, 포인터 변수)는 주소값을 볼 순 없지만 raw pointer를 이용하면 확인할 수 있다.

이외에 아래와 같은 성질이 있다.
1. 빌림 규칙을 무시할 수 있으며, 불변 가변을 동시 혹은 여러개의 가변을 가질 수 있다.
2. 유효한 메로리를 가리키는 것에 대한 보장을 받지 못함
3. null이 될 수 있음
4. 자동 메모리 정리가 되지 않음

```rust
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}
```
위와같이 원시포인터를 생성 및 역참조

원시포인터는 C언어와 상호작용할 때 쓰인다고 함

## extern 외부 코드 호출

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}
```
C에는 ABI(Application Binary Interface)를 통해 abs함수를 제공한다.

반대로 Rust에서 다른 언어로도 제공할 수 있는데 이 때는 `#[no_mangle]`키워드를 붙인다.
> 맹글링은 컴파일러가 최적화 과정중 보다 많은 정보를 제공하기 위해 함수명을 변경하는데, 이 과정을 맹글링이라고 함

# 고급 트레잇

## 연관 타입

연관 타입은 트레잇 메서드를 정의할 때 시그니처에서 임시로 구체적인 타입을 명시하지 않더라도 사용할 수 있게 해주는 타입이다.

사실 쉽게말하면 그냥 타입을 확정짓지 않는 별칭이라고 보면된다.

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
    }
}
```
위와 같이 구현부에서 타입을 지정할 수 있다. 물론 제네릭과 엮어서도 사용할 수 있고, 구지 Trait에서 사용안해도 된다. ( 이때는 이제 단순 별칭역할만 )

> 제네릭과의 차이점
> 
> 제네릭 : 동일한 구조체, 동일한 특성에 대해 여러번 구현 수 있게 해줌. ex) Add<Point> for Point, Add<Vector> for Point
> 
> ( 보통 사용하는 측에서 타입을 특정하기에 입력 핸들링 )
> 
> 연관타입 : 동일한 구조체, 동일한 특성에 대해 단 한번만 구현할 수 있게 해줌. ex) Add for Point
> 
> ( 보통 구현부쪽에서 타입을 특정하기에 출력 핸들링 )

## 연산자 오버라이딩

Add와 같은 특정 Trait을 구현하면 연산자 오버라이딩을 할 수 있다.

## 완전정규화 구문

트레잇 메서드의 중복 이름 및 둘다 구현이 가능하다. 따라서 이름이 모호한 상황이 발생하는데, 이 때 완전정규화를 사용해야 한다.

```rust
fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
```
사용처에서 `구체 as Trait`를 사용해 어떤 Trait의 메서드를 사용할지 알려주어야 한다.

## 외부타입 트레잇 구현

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```
Trait구현 규칙에는 고아규칙이 있는데, 이 때문에 외부 Crate에 있는 객체에 대해 외부 Trait을 적용할 수 없다.
이 때 위 처럼 Wrapping 해주면 사용할 수 는 있긴하다.

# 고급 타입

## 타입 별칭

```rust
type Kilometers = i32;
```

## 빈 타입

`!`라는 타입이 있는데, 이는 절대 반환하지 않는 타입이라고 보면된다.
가령 무한 루프가 도는 함수가 있는데, 이럴때 반환타입이 `!`인 것

## 동적 크기 타입

기본적으로 러스트는 컴파일시점에 사이즈가 결정되어야 하지만 종종 런타임 시점에 사이즈 결정이 필요할 수 있다. 이러한 타입을 unsize type이라고 부른다.

이러한 unsize type은 객체는 Runtime 시점에 만들고 Compile시점에는 포인터변수로 사용하여, 해당 주소에 대한 사이즈만을 계산하게 한뒤 넘기는 방식으로 처리한다.

제네릭같은경우 일반적으로 size type에 대해서만 추상대역을 제공하는데, T: ?Sized 즉 Sized Trait이거나 아니거나로 설정하면 동적 크기에 대한 타입도 받을 수 있으며,
이때는 &T로 사용해야 한다.
> Sized Trait은 컴파일 시점에 타입의 크기를 알 수 있는지 여부를 결정한다. 일반적으로 제네릭은 T = T: Sized와 동일하다고 보면된다.

# 고급 함수

## 함수 포인터

```rust
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
```
위와 같이 함수도 인자로 넘길 수 있다. fn의 경우 Fn, FnMut, FuOnce모두를 전부다 구현한다.

## 클로저 반환

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```
클로저는 unsize type이므로 위와같이 Box와 Trait을 사용해서 반환해주어야 한다.

# 매크로

매크로는 단순하게 code generator의 역할이라고 보면된다. ( 메타 프로그래밍 )

매크로 같은경우 선언적 매크로와 세 가지의 절차적 매크로가 있다.

## 선언적 매크로

`macro_rules!` 구문을 사용해 정의할 수 있다.

## 절차적 매크로

절차적 매크로는 아래와 같이 세 가지가 있다.

1. 구조체와 열거형에 사용되는 derive속성이 추가된 #[derive] 매크로 ( 보통 Trait의 기본구현을 많이 사용하는 듯 )
2. 모든 아이템에 사용가능한 커스텀 속성을 정의하는 속성형 매크로 ( #[route(GET, "/")]와 같은 매크로로 모든 곳에서 사용 가능 )
3. 함수 호출처럼 보이지만 지정된 토큰을 인수로써 사용하는 함수형 매크로 ( println!가 여기에 속함, 함수 호출처럼 보임 )