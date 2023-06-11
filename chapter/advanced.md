# 고급

## Generic

제네릭은 Concrete타입에 대해 추상화된 대역이다.

```rust
fn trans<T>(content: T) -> T {
    content
}

struct Point<T> {
    x: T,
    y: T,
}

enum Option<T> {
    Some(T),
    None,
}
```

구조체, 열거형, 함수, 메서드 등에 대해서 제네릭을 걸 수 있다.

```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn trans<U>(self, other: Point<U>) -> Point<U> {
        Point {
            x: other.x,
            y: other.y,
        }
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

1. impl에 대해 제네릭 타입을 지정할 수 있다. ( 구조체의 타입을 따라 감 )
2. 메서드에도 제네릭타입을 지정할 수 있으며 구조체와는 별개의 타입을 가짐
3. impl에 concrete타입을 지정할 수 있으며, 이를 통해 해당 타입에 대해서만 메서드를 노출시킬 수 있다.

## 성능

자바에서는 제네릭소거라는 개념을 통해 제네릭의 타입을 컴파일시점에 소거하여버린다. 즉 List<T>는 List가 되는 것

하지만 Rust에서는 제네릭을 컴파일시점에 소거하지 않는다. 대신 컴파일시점에 제네릭을 구체화시켜버린다.

즉 Point<T>라는 구조체를 사용한다면, 이를 사용하는 모든 곳을 찾아가 타입을 특정화한다. 이를 러스트에선 단형성화(monomorphization)이라고 한다.

따라서 위 단형성화를 통해 성능을 보장할 수 있게된다.

## Trait

트레잇은 공통된 기능을 추상적으로 정의할 수 있게 해준다.
> 트레잇은 자바에서의 interface와 흡사한 기능을 한다.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```
트레잇은 위와 같이 정의할 수 있다.

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```
사용법으로는 impl뒤에 트레잇을 적고 for로 어떠한 타입에 대해 트레잇을 적용할지 선택한 다음 해당 메서드를 구현해주면 된다.

> Trait을 구현할때는 항상 Trait이 해당 크레이트에 있거나, 구현할 대상 struct가 해당 크레이트에 있어야만 한다.
> 
> 이러한 제약조건은 내가 작업한 Trait과 그 구현 struct에 대해 외부 코드로부터 안전하게 하기 위함이다.

### Override & Default

```rust
trait Summary {
    fn summarize(&self) -> String;
    
    fn default(&self) -> String {
        self.summarize()
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    
    fn default(&self) -> String {
        let mut string = String::from("Override : ");
        string.push_str(self.summarize().as_str());
        string
    }
}

impl Summary for Vec<i32> {
    fn summarize(&self) -> String {
        format!("Vec<int32> length: {}", self.len())
    }
}
```

오버라이드와, 디폴트 메서드또한 정의할 수 있다.

```shell
Override : horse_ebooks: of course, as you probably already know, people
Vec<int32> length: 3
```

해당 코드의 출력결과는 위와 같다.

### 매개변수 및 Trait Bound

```rust
fn print_summary(item: &impl Summary) {
    println!("{}", item.summarize());
}
```
위 처럼 Summary를 구현한 struct에대 해서만 파라미터로 받도록 가능하다.

```rust
fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}
```
Trait Bound는 위와같은 형태이며, impl Trait을 사용하는 것과 동일하다. ( 정확히는 impl Trait이 Trait Bound의 Syntax Sugar이다. )

#### 복수의 Trait Bound
```rust
fn print_summary<T: Summary + Display>(item: &T) {
    println!("{}", item.summarize());
}
```
이를 통해 T는 Summary와 Display를 전부다 구현한 타입이도록 제한할 수 있다.

이 때 Trait Bound의 제약사항이 많아지면 코드가 엄청 난잡해지는데 이를 `where` 문법으로 쉽게 해결할 수 있다.

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32

fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
```
위 2개의 코드는 동일한 역할을 한다.

### Trait 타입 반환

```rust
fn create_tweet() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```
위처럼 impl Trait을 통해 Trait 구현 Type을 반환할 수 있다.

하지만 impl Trait을 사용한다 하더라도, 다양한 구현타입을 반환할 수 없다. ( 제약조건이 걸려있음 )

### 포괄구현

```rust
impl<T: Display> ToString for T {
    // --snip--
}
```
위와같이 Display를 구현한 모든 타입에 대해 ToString메서드를 사용할 수 있게 하는 행위를 포괄구현(blanket implementations)이라고 한다.

## 라이프타임

모든 참조자는 라이프타임을 갖고 있으며, 이를 통해 유효성을 보장 함

### 댕글링 포인터

```rust
let x;
{
    let y = 5;
    x = &y;
}
println!("{}", x);
```
이 코드는 지난 소유권부분에서 봤던 `댕글링 포인터`이다. 이러한 에러가 발생하는 이유는 러스트의 독특한 메모리 관리법 때문인데, 해당 스코프가 끝날때 메모리에서 값을 해제하기 때문이다.

이러한 특성을 Rust에서는 lifetime이라 부르는데, 위 코드를 다시 예로 보면

let x의 라이프타임은 코드의 전체적인 부분이나, y의 라이프타임은 중괄호 내부로 한정된다. 즉 lifetime의 크기가 x가 더 큰걸 알 수 있다.

따라서 println에서 x를 호출할때 y의 라이프타임이 끝났으니 해당 값은 더이상 유효하지 않다는 에러가 발생하는 것이다.

### 문제 케이스

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

Case #1

fn main {
    let string1 = String::from("long string is long");
    let string2;
    {
        let string = String::from("xyz");
        let string2 = &string;
    }
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}

Case #2

fn main {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```
위 코드도 댕글링 포인터와 마찬가지로 라이프타임에 의해 에러가 발생한다.

1. Case#1번부터 보자 댕글린 포인터의 **메서드 확장버전**이다. lognset함수에서 x, y를 사용하려보니 y에 대한 라이프타임이 끝나있다.
2. Case#2번을 보면 정상적으로 함수는 호출이 되었으나, result값을 사용하려고 보니 라이프타임이 끝나있는걸 알 수 있다.

즉 기본적으로 간단한 라이프타임의 경우 컴파일러가 추론할 수 있으나, 위와 같이 함수나 메서드사용과 같이 복합적으로 간다면 이에 대해 추론하기가 힘들다.

따라서 이럴때 해당 함수에 대해 이 특정 기간동안 유효함을 보장해주는 라이프타임이 명시적인 선언이 필요하다.

### 해결법

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
어려운 내용은 없다 제네릭에 '가 붙은 대역을 선언해주고 &키워드 앞에 붙여주면 된다. 이렇게 되면 해당 함수를 호출하는 중에 'a라는 라이프타임으로 x, y, return값이 유효하다는걸 컴파일러에게 알릴 수 있다.

라이프타임을 명시하지 않은 함수에 대해선 IDE lint에 의해 1차적으로 걸리게되며, 라이프타임을 명시했을 때 Case#1, Case#2에 대한 호출시 패닉이 발생하게 된다.

### 구조체에서 라이프타임 사용

```rust
struct LifetimeStruct<'a> {
    data: &'a str,
}
```
LifetimeStruct라는 구조체는 data라는 필드가 참조하고 있는 값의 라이프타임보다 오래 살 수 없다라는 의미이다.

만약 'a, 'b이런식으로 각각의 라이프타임이 있고 data1: 'a, data2: 'b라면 결국 data1, data2라는 값의 라이프타임보다 오래 살 수 없는 것 이다.

### 라이프타임 생략 규칙

프로그래머가 라이프타임을 명시할 때 몇 가지 특정 패턴이 보이는데, 이를 Rust컴파일 과정중 패턴화시켜 생략시킬 수 있도록 하였다.

라이프타임 생략 규칙은 다음과 같다.

1. 파라미터가 n개라면 함수는 n개의 라이프타임을 갖는다.
2. 입력 라이프타임 파라미터가 1개라면 출력 라이프타임은 입력과 라이프타임이 공유된다.
3. 여러 매개변수중 &self, &mut self가 속해 있다면, self의 라이프타임이 모든 출력 라이프타임의 라이프타임이 된다.

```rust
fn first_word(s: &str) -> &str

fn first_word<'a>(s: &'a str) -> &str // 첫 번째 규칙

fn first_word<'a>(s: &'a str) -> &'a str // 두 번째 규칙

impl<'a> test<'a> {
    fn test(&self, test_parameter: &str) -> &str {
        self.data
    }
} // 세 번째 규칙에 의해 self lifetime이 반환 됨
```
위와 같이 각 규칙별로 라이프타임이 컴파일 과정중 정의된다.

### 정적 라이프타임

`'static`이라는 키워드가 붙으면 프로그램의 전체 생애주기동안 살아있음을 의미한다.

