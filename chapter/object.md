# 객체지향

## 객체지향의 특성

### 캡슐화

pub 혹은 pub(crate)를 통해 정보은닉 및 캡슐화를 할 수 있다.

### 상속 및 다형성

러스트는 공식적으로 상속을 지원하지 않는다, 이를 구현하려면 매크로를 사용해 부모의 피륻와 메서드 구현을 상속받는 구조체를 만드는 것 말곤 없다.

다만 Trait을 통해 메서드를 부분적으로 공유할 수 있다. 그리고 이를 통해 다형성을 구현할 수 있다.
> 정확히는 제네릭 & Trait Bound를 통해 가능 함

## 트레잇 객체 활용

Vector는 하나의 타입만 가질 수 있다. 정확히는 일반적인 제네릭변수는 하나의 변수 대역만 가지고있는데, 트레잇 객체를 사용하면 이를 극복할 수 있다.

```rust
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```
먼저 스마트 포인터인 Box타입안에 Trait객체를 넣는다. 이 때 Trait 객체를 Generic에서 사용할 땐 dyn키워를 붙인다.

이렇게 하게되면 추 후 Vector에는 `Box<Image>, Box<TextBox>, Box<Button>`을 넣게될 수 있다.

### 동적 디스패치

Generic은 기본적으로 컴파일 시점에 T라는 추상적인 대역을 구체적인 타입으로 바꾸는 `단형성화`를 진행하는데, 이 과정중에서 코드 인라인 및 최적화가 이루어진다.
그리고 이렇게 단형성화된 코드는 컴파일 시점부터 어떠한 구현체의 메서드를 호출할지 즉 어느 곳으로 디스패치할지에 대해 명확히 규정이 되어잇기에 `정적 디스패치`가 이루어지는반면

트레잇을 제네릭에서 사용하게 되면, 컴파일 시점에 어떠한 트레잇인지 알 수 없어 런타임시점으로 넘어가게 되고 ( 이 때는 포인터로 조회해서 확인할 수 있음 )
이렇게 런타임 시점에 어떠한 구현체로 디스패치 할지 여부를 결정하는 것을 동적 디스패치라고한다. 당연히 최적화 및 런타임에서의 추가 비용이 발생한다.

정리하면 Primitive, Struct type의 경우 정적 디스패치, Trait의 경우 동적 디스패치를 사용한다.
