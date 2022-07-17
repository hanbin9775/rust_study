use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;

fn main() -> Result<(), Box<dyn Error>> {
    let v = vec![1, 2, 3];
    // v[99]; // panic! 호출.

    // panic 이 호출되면 RUST_BACKTRACE 환경 변수로 정확히 에러가 어떻게 발생했는지 추적할 수 있다.
    // RUST_BACKTRACE=1 cargo run

    let f = File::open("hello.txt"); // return type: Result

    // Result type을 처리하는 방법
    let f = match f {
        Ok(file) => file,
        Err(ref error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("파일을 생성하지 못했습니다: {:?}", e),
            },
            other_error => panic!("파일을 열지 못했습니다: {:?}", other_error),
        },
    };

    // match 표현식은 코드가 길어지므로 Result type은 unwrap, expect 메서드를 제공한다.

    // unwrap: Result Ok -> Ok 에 저장된 값 리턴, Err -> panic! 호출
    // let f2 = File::open("hello2.txt").unwrap();

    // expect: panic! 을 호출할때 함께 출력할 문자열도 함께 넘겨준다.
    // unwrap을 많이 쓰면 unwrap 메서드가 같은 에러 메시지를 출력하므로 어떤 곳에서 호출되었는지 확인이 어려움.
    let f3 = File::open("hello3.txt").expect("파일을 열 수 없습니다.");
    let f4 = File::open("hello.txt")?;
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut s = String::new();
    // ? 연산자는 Result 값이 Ok 이면 Ok enum 에 저장된 값 리턴
    // Err 이면 Err 열거값이 return 키워드를 사용한 것 처럼 전체 함수의 리턴값이 되어 호출자로 에러가 전파
    // 에러값이 표준 라이브러리에 정의된 From 트레이트에 선언되어 있으며 에러를 어떤 한 타입에서 다른 타입으로 변환한다.
    // ? 연산자는 Result type을 리턴하는 함수에 대해서만 사용 가능
    // let mut f = File::open("hello.txt")?;
    // f.read_to_string(&mut s)?;

    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    fs::read_to_string("hello.txt")
}

// 다음과 같은 구조체를 정의하여 값에 대한 유효성 검사를 할 수 있다.
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("유추한 값은 반드시 1~100 사이의 값");
        }

        Guess { value }
    }

    // getter
    pub fn value(&self) -> i32 {
        self.value
    }
}
