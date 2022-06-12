#[derive(Debug)] // debug 특성 추가.
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)] // debug 특성 추가.
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)] // debug 특성 추가.
enum Message {
    // 각기 다른 구조체 타입을 정의하는 것과 유사
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    // imp1 블록을 이용해 열거자에 메서드를 정의할 수 있다
    fn call(&self) {
        println!("{:?}", self)
    }
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4(127, 0, 0, 1),
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6(String::from("::1")),
        address: String::from("::1"),
    };

    println!("{:?}", home.kind);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5); // 어떤 타입의 데이터도 저장할 수 있다.println!
    let some_string = Some("string");
    let absent_number: Option<i32> = None; // Option<T> 와 T 는 다른 타입이기 때문에 컴파일러는 유요한 값이 명확히 존재할 때는 Option<T> 값을 사용하는 것을 허락하지 않는다.

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    value_in_cents_with_if(Coin::Quarter);
    value_in_cents_with_if(Coin::Dime);
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1, // 표현식
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents_with_if(coin: Coin) -> u32 {
    if let Coin::Quarter = coin {
        println!("Quarter");
        25
    } else {
        println!("else!");
        5
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // match 표현식은 반드시 모든 경우를 처리해야 한다.
        None => None,
        Some(i) => Some(i + 1),
        // _ => Some(1), 별도의 처리가 필요하지 않는 경우 _ 자리 지정자로 대체하면 된다
    }
}
