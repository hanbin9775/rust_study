# 범용 컬렉션

- 벡터: 연속된 일련의 값을 저장한다.
- 문자열: 문자의 컬렉션. 보통 String 타입과 문자열 슬라이스 &str 타입을 동시에 의미한다.
- 해시 맵: 특정 키에 값을 연결할 때 사용한다. 해시 맵은 더 범용으로 사용되는 맵을 구현한 구현체이다.

## 문자열의 인덱스

러스트의 문자열은 인덱스를 지원하지 않는다.

### 문자열의 내부

```
let len1 = String::from("Hola").len(); // 4

let len2 = String::from("안녕하세요").len(); // 15
```

len2 는 15를 할당받는다. 이는 문자열 안 유니코드의 스칼라 값은 3byte 공간을 차지하기 때문이다.

### 바이트와 스칼라값, 그래핌 클러스터

러스트 관점에서 볼 때 문자열은 크게 바이트, 스칼라값, 그래핌 클러스터 3가지로 구분한다.

러스트가 String 타입에서 인덱스 사용을 지원하지 않는 마지막 이유는 인덱스 처리에는 항상 일정한 시간이 소요되어야하지만 String 타입에 대해서만큼은 일정한 성능을 보장할 수 없어서다. 왜냐하면, 러스트는 유효한 문자를 파악하기 위해 콘텐츠를 처음부터 스캔해야 하기 때문이다.
