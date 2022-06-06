fn main() {
    {
        // 프로그램 안에 하드코딩한 무자열값인 무자열 리터럴을 참조한다.
        let s = "hello"; // 변수 s는 이 지점부터 유효하다.
                         // 변수 s를 이용해 필요한 동작을 수행한다.
        println!("s: {}", s)
    } // 여기서 범위를 벗어나므로 변수 s는 이제 유효하지 않다.
      // String 타입은 힙에 할당되므로 컴파일 시점에 알 수 없는 크기의 문자열을 저장할 수 있다.
      // :: 은 from 함수를 String 타입의 이름 공간으로 제한해서 더 명료한 이름을 사용하기 위해서
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // 값 5를 변수 x에 대입한 후, 변수 x 값의 복사본을 변수 y에 대입
    let x = 5;
    let y = x;

    // String 타입은
    // 문자열 콘텐츠를 저장하는 메모리를 가리키는 포인터,
    // 갈아, 용량 세 부분으로 구성된다
    // s1을 s2에 대입하면 String 타입의 데이터가 복사된다.
    // 즉 포인터가 가리키는 힙 메모리의 실제 데이터가 아니라 문자열에 대한
    // 포인터와 길이, 요량이 스택에 복사된다.
    // 이는 메모리를 해제할때 이중 해제 에러를 일으킬 가능성이 있으므로,
    // 러스트에서는 s2 = s1을 한 후 다시 s1을 참조하는 것을 허용하지 않는다.
    let s1 = String::from("hello");
    let s2 = s1; // 러스트에서는 실제 데이터를 복사하지 않고 메모리를 이동 시킨다.
                 // println!("{}", s1); // Error!

    // 딥 복사를 원한다면 clone 메서드 사용
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // stack 에 할당되는 데이터는 아무 문제 없다.
    // 러스트에서 이러한 타입은 Copy 특성을 지닌다.
    // 통상적으로 단순한 스칼라 값에 Copy 특성이 부여된다.
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");
    takes_ownership(s); // 이 시점부터 변수 s는 유효하지 않음.

    let x = 5;
    makes_copy(x); // copy 하므로 x는 계속 유효함.

    let s1 = gives_ownership();

    let s2 = String::from("hello");
    // s2의 소유권이 s3로 이전
    let s3 = takes_and_gives_back(s2);

    let s1 = String::from("hello");

    // 대여
    // 값의 소유권을 가져오는 대신, 매개변수로 전될된 객체의 참조를 이용하도록 다시 작성할 수 있다.
    let len = calculate_length(&s1);

    println!("{} 의 길이는 {} 이다", s1, len);

    let mut s = String::from("hello");
    // 가변 참조
    change(&mut s);

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    // 불변 참조를 이미 사용 중일 때, 가변 참조를 생성할 수 없다.
    // let r3 = &mut s;

    let reference_to_nothing = dangleFix();

    let s = String::from("Hello");

    println!("{}", first_word(&s));

    let s = String::from("hello world!");

    let word = first_word_with_slide(&s);
    // s.clear(); 위에서 불변참조가 이루어져서 가변 참조가 필요한 clear 함수를 호출할 수 없다

    println!("{}", word);

    // s의 타입은 &str(슬라이스 타입) 이고 불변 참조이기 때문에 항상 불변이다.
    let s = "Hello, world!";
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// 참조하고 있는 값은 변경할 수 없다.
fn calculate_length(s: &String) -> usize {
    // s.push_str(", world!"); ERR
    s.len()
}

fn change(s: &mut String) {
    let r1 = s;
    // 다만 특정 범위 내의 특정 데이터에 대한 가변 참조는 오직 한개만 존재해야 한다.
    //let r2 = s;  ERR!
    r1.push_str(" world!");
}

// 죽은 참조: 이미 해제되어 다른 정보를 저장하도록 변경된 메모리를 계속해서 참조하는 포인터
// 이 함수의 리턴 타입은 대여한 값을 리턴하지만, 실제로 대여해 올 값이 존재하지 않는다.
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s // 변수 s가 범위를 벗어나므로 drop 함수가 호출되고 메모리가 해제된다.
// }

fn dangleFix() -> String {
    let s = String::from("hello");
    s
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    println!("{:?}", bytes);

    // enumerate 메서드는 튜플을 리턴하므로 다른 러스트 코드에서와 마찬가지로 튜플을 해제하는 패턴을 사용하면 된다.
    // iter().enumerate() 메서드가 리턴하는 요소에 대한 참조를 얻어와야 하므로 패턴에서 & 기호를 사용
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // 공백을 표현하는 바이트 검색
            return i;
        }
    }
    s.len()
}

fn first_word_with_slide(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
