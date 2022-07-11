use std::collections::HashMap;
fn main() {
    let v1: Vec<i32> = Vec::new();

    // vec! 매크로를 사용하면 초기값을 유추하여 타입 애노테이션을 추가해줄 필요가 없다.
    let v1 = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    println!("{:?}", v);

    {
        let v = vec![1, 2, 3];
    } // 다른 구조체와 마찬가지로 범위를 벗어날 때 drop 메서드가 호출되며 메모리가 해제된다.

    let v = vec![1, 2, 3, 4, 5];

    // 직접 접근하는 방식에서 범위를 벗어난 인덱스로 접근하면 패닉이 발생
    // let num: &i32 = &v[100];

    // get 메서드는 Option<&T>을 반환한다
    // 입력 오류 때문에 프로그램을 강제 종료하는 것보다는 훨씬 사용자 친화적인 방법이라고 할 수 있다.
    match v.get(100) {
        Some(element) => println!("백 번째 원소: {}", element),
        None => println!("백 번째 원소가 없습니다."),
    }

    let mut v2 = vec![1, 2, 3, 4, 5];
    let first = &v2[0];
    v2.push(6);
    println!("{:?}", v2);

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    // 열거자를 사용하여 원소마다 다른 타입을 지닌 벡터를 생성할 수 있다.
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("블루")),
        SpreadsheetCell::Float(10.12),
    ];

    // 새로운 빈 문자열을 생성해 변수 s에 할당한다.
    let mut s = String::new();

    // s1, s2, s3 모두 동일하다
    let data = "문자열 초기값";
    let s1 = data.to_string();
    let s2 = String::from("문자열 초기값");
    let s3 = "문자열 초기값".to_string();

    println!("{} {} {}", s1 == s2, s2 == s3, s1 == s3);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    // push_str 메서드는 매개변수의 소유권을 가질 필요가 없다.
    s1.push_str(s2);
    // s2 는 계속 사용 가능
    println!("s1: {}, s2: {}", s1, s2);

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s1 은 메모리가 해제되어 더 이상 사용할 수 없다.
    let s3 = s1 + &s2;
    println!("{}", s3);

    // + 연산자는 add 메서드를 사용하는데, 이 메소드의 시그니처는 다음과 같기 때문
    // fn add(self, s: &str) -> String {}. 정확히 일치 x
    // add 메서드를 호출하면 러스트는 강제 역참조를 이용하여 &s2를 &s2[..]로 변환한다.
    // s1의 소유권을 확보하고 s2의 값을 복사해서 덧붙인 후 그 결괏밗에 대한 소유권을 리턴한다.
    // 이 방식은 계속 값을 복사하는 것보다 훨씬 효율적이다.

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3; // hard to read
    let s = format!("{}-{}-{}", s1, s2, s3); // good!

    for c in "안녕하세요".chars() {
        println!("{}", c);
    }

    for b in "안녕하세요".bytes() {
        println!("{}", b);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("블루"), 10);
    scores.insert(String::from("옐로"), 50);

    let field_name = String::from("Favorite color");
    let field_value = String::from("블루");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // 이후에는 사용할 수 없다.field_value

    let name = String::from("블루");
    let score = scores.get(&name); // score 는 Option 타입이다.
    println!("{} {}", name, score.unwrap());

    for (key, value) in &scores {
        println!("{} {}", key, value);
    }
}
