# 열거자

### Option 열거자

표준 라이브러리가 제공하는 열거자. 어떤 값이 존재하거나 존재하지 않는 범용적인 시나리오에 적합하도록 디자인되었다. 이는 러스트가 모든 경우의 수를 처리하고 있는지를 컴파일러가 확인할 수 있다는 것을 의미한다.

### Null

러스트는 null 이라는 값의 개념이 없다. 널의 창시자인 토니호어조차 널이 에러, 보안 취약, 시스템 충돌 등의 이유로 '십억달러 짜리 실수' 라고 말한다. 널값의 문제는 널값을 널이 아닌 값처럼 사용하려고 하면 에러가 발생한다.

하지만 널값을 이용해 표현하려고 했던개념은 지금도 여전히 유용하다. 사실 문제는 널의 개념이 아니라 그 구현에 있다. 그래서 러스트는 널값이라는 개념이 없지만, 어떤 값의 존재 여부를 표현하는 열거자를 정의하고 있다. 이 열거자가 바로 Option<T>이며 표준 라이브러리에 다음과 같이 정의되어있다.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

이 열거자는 프렐류드에도 포함되어있어 열거자는 물론 그 안에 열거된 값도 명시적으로 범위로 가져올필요가 없다.
