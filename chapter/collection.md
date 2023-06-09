# 컬렉션

러스트의 prelude에는 여러가지 유용한 컬렉션들이 포함되어 있다. 컬렉션은 유용한 데이터 구조를 가진 자료구조들의 집합이다.

## 벡터

벡터는 하나의 타입을 갖는 데이터 셋으로 가변적인 성질을 가진다.

만약 벡터가 버려지게 되면, 벡터가 들고 있는 모든 요소가 버려진다.

```rust
let v: Vec<i32> = Vec::new();

v.push(5);
v.get(0);
&v[0];
```
벡터는 위와같이 사용할 수 있다.

이 중 벡터에서 값을 조회할 때 2가지의 경우가 있는걸 볼 수 있다.

1. v.get()의 경우 Option<?>을 뱉으며 이는 값이 없더라도 패닉이 아닌 None으로 처리할 수 있도록 한다
2. &v[0]의 경우 값이 없을 경우 패닉을 일으킨다.

```rust
fn main() {
    let v = vec![1, 2, 3];

    let x = &v[0];
    let y = v.get(1);

    v.push(5);
}
```
x, y의 경우 불변참조자이며 push의 경우 가변참조자인데, 러스트의 빌림 원칙에 의해 push는 사용할 수 없다.

### 반복 처리

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{i}");
}

let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```
위 처럼 가변참조조와 불변참조자를 사용해 반복할 수 있다.
이 중 가변의 경우 역참조를 통해 실제 값을 증가시키고 있다.

### 열거형을 이용한 다양한 타입 처리

```rust
enum Tuple {
    Int(i32),
    Float(f64),
    Text(String),
}

let vec = vec![Tuple::Int(5), Tuple::Float(5.0), Tuple::Text(String::from("Hello"))];
```

위 처럼 열거형을 이용해 다양한 타입을 지원할 수 도있다.

## 문자열

1. 문자열은 바이트 컬렉션의 집합이며 동시에 UTF-8을 인코딩된다.
2. String은 문자열이고 &str은 문자열 슬라이스다.

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1은 더이상 사용할 수 없음 s3로 소유권이 이전 됨

fn add(self, s: &str) -> String {
}
```

문자열은 + 연산자를 통해 더할 수 있는데 이는 밑 add함수를 호출한다. 이 때 재밌는 점이 2가지가 있는데,

1. 역참조
> s1 + &s2의 경우 결국 String + &String인건데, add에서 받는 파라미터는 &str으로 엄연히 &String과 다르다.
>
> 이는 러스트에서 역참조 강제라는 내용으로 &s2를 강제로 &s2[..] 슬라이스로 바꾸어 새롭게 복사를 한다.

2. 소유권
> add의 self를 보면 참조가가아닌 소유권을 가져가고있다. 따라서 s1의 원래의 소유권이 새로운 문자열로 이전이된다.
> 즉 s1의 소유권을 가져다가 s2의 복사본을 합친 뒤 새로운 문자열을 반환한다.

## 인덱싱

1. 러스트는 문자열에 대해 인덱싱을 지원하지 않는다.
2. String은 단순 Vec<u8>을 감싼 것 이다.

타 언어에서는 인덱스를 문자단위로 구성했는데, 러스트의 문자열은 바이트 배열이다.

따라서 바이트의 크기가 가변적으로 늘어나는 UTF-8형식의 인코딩을 기본으로 사용한 러스트는 문자열 인덱스를 사용할 수 없다.

### 바이트 스칼라, 자소

정윤성이라는 문자를 바이트로 표현한다면

`236 160 149 236 156 164 236 132 177`와 같다.
이를 스칼라로 표현하면 `정윤성`이 된다. 자소로 본다면 발음기호와 같은 필요없는 단어를 전부 뺀 하나의 글자를 의미한다.

1. 바이트 = 8비트 크기
2. 스칼라 = char로 1byte의 크기
3. 자소 = 실제 사용되는 글자

```rust
for b in hello.chars(){
    println!("{b}");
}

for b in hello.bytes() {
    println!("{b}");
}
```
위 코드를 통해 바이트와 스칼라를 확인할 수 있다.

### 슬라이싱

만약 정말로 인덱스가 필요하다면 슬라이싱을 사용하자.

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

정상적으로 값이 출력된다. 만약 유니코드의 중간 부분을 자른다면 즉 &hello[0..1]을 사용한다면 에러를 마주할 것 이다.

## 해시맵

Key와 Value를 쌍으로 갖으며 해시 함수를 사용해 매핑하는 자료구조이다.

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
let score = scores.get(&"Blue");
scores.entry(String::from("Yellow")).or_insert(50);

for (key, value) in &scores {
    println!("{key}: {value}");
}
```

위와 같이 적재 및 조회를 할 수 있다.

entry의 경우 값이 없으면 50을, 있으면 해당 값을 리턴한다. 또한 or_insert의 경우 가변참조자를 리턴한다.

### 소유권

1. Copy Trait을 구현한 타입들은 값을 복사한다.
2. 그 외의 타입은 소유권을 HashMap으로 이전한다.

```rust
let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name과 field_value는 이 시점부터 유효하지 않습니다.
```
