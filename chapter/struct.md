# 구조체

여러 타입을 가질 수 있다는 점에서 튜플과 흡사하나, 
각 요소에 이름을 붙일 수 있으며 하나의 구조안에 연관된 데이터를 묶음으로 인해 응집도도 높아진다는 점에서 차이가 있다.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

구조체는 아래와 같이 인스턴스를 만들 수 있다

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

구조체는 일부 필드만 가변적으로 사용할 수 없으며 구조체 전체가 가변적이거나 불변적 이어야 한다.

## 구조체 업데이트

```rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

위와같이 user1이라는 User인스턴스를 통해 user2를 만들어 낼 수 있다.

당연하겠지만 이후 user1인스턴스는 더 이상 사용할 수 없다. active, sign_in_count는 모두 Copy 트레잇을 구현한 타입이므로, 스택에서 데이터 복사가 이루어진다.

## 튜플 구조체

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

위와같이 이름과 타입만 갖고 있는 튜플형 구조체도 만들 수 있다.

## 유닛 구조체

```rust
struct AlwaysEqual;
```

아예 필드가 존재하지 않는 유닛 구조체 또한 있다.
이는 단순하게 어떤 타입에 대해 트레잇을 구현하고 싶지만 데이터는 필요없을 때 사용할 수 있다.

## 메서드

### 정의

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```
`impl`블록안에 함수를 정의하면 해당 구조체의 메서드로서 사용할 수 있다

```rust
fn area(&self) // 불변 참조자
fn area(&let mut self) // 가변 참조자
fn area(my_self: &self) // 별칭
```
중요한것은 메서드에서 항상 첫 번째 매개변수는 &self로 와야한다. 만약 이름을 변경하고 싶다면 name: &self로 바꿀 수도 있다.
때에 따라선 &self를 &mut self로 수정하여 가변참조자로서도 사용할 수 있다.

> 역참조
>
> 러스트에서는 C++에서의 -> 연산자 즉 역참조 연산자를 따로 지원하지 않는다.
> 
> p1.distnace(&p2), (&p1).distance(&p2)
> 이와같이 자동참조 및 역참조라를 지원한다.
> 
> 역참조 : 참조의 경우 (주소) -> (값) 즉 값을 주소가 참조한다면, 역참조의 경우 (값) -> (주소) 즉 주소가 값을 참조한다.

## 연관 함수

impl내에 self를 가지지 않는 함수를 만들 수 있는데, 이는 구조체와 연관이 있다하여 연관함수라고 한다.

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}
```

연관함수의 경우 생성자 즉 팩토리메소드로서 활용이 된다.