fn main() {
    // rust 변수는 immutable 이 default
    let mut x = 5;
    println!("x의 값: {}", x);
    x = 6;
    println!("x의 값: {}", x);

    let shadow_x = 5;
    let shadow_x = shadow_x + 1;
    let shadow_x = shadow_x * 2;
    println!("shadow_x의 값: {}", shadow_x);

    let spaces = "   ";
    let spaces = spaces.len();
    // spaces = spaces.len(); type err! 변수 선언시 타입이 정해진다
    println!("spaces의 값: {}", spaces);

    // 여러 타입으로 추론이 가능할 때는 타입을 명시해주어야 한다.
    let guess: u32 = "42".parse().expect("숫자가 아닙니다!");


    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup; // 구조분해
    println!("y의 값: {}", y);

    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // 힙영역이 아니라 스택 영역에 할당. 고정된 길이, 타입 자료 구조를 표현하는데 유리

    let arr = [3; 5];
    // 컴파일시 에러가 발생하지 않지만, 런타임 에러가 발생 (패닉) (응? 근데 내 환경에서는 컴파일시 에러가 발생함..)
    // 러스트의 안정성 원리. 대부분의 저수준 언어는 이러한 검사를 하지 않는다.
    // 러스는 엉뚱한 메모리에 대한 접근을 허용하지 않고, 즉각 중단하는 보호 장치를 제공한다.
    // let element = arr[10]; 

    simple_function(5, 6);

    println!("func five: {}", five());

    let number = 3;

    if number < 5 { // if 표현식의 조건을 항상 불리언으로 제공해야함. 자동 변환 X
        println!("조건 일치!");
    } else {
        println!("조건 불일치!");
    }

    // if 도 표현식이므로 let 구문 오른쪽에 사용할 수 있다.
    // if, else 에서 반환하는 타입이 항상 같아야 한다
    let number = if true {
        5
    } else {
        6 
    };
    println!("if 표현식 반환값: {}", number);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result: {}", result);

    let a = [1, 2, 3, 4, 5];

    for element in a.iter() {
        println!("요소의 값: {}", element);
    }
}

fn simple_function(x: i32, y: i32) {
    println!("x, y: {} {}", x, y);
}

fn five() -> i32 {
    5
}
