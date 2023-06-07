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

