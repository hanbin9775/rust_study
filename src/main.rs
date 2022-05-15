use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자 맞추기");

    let rand_num = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("정답이라고 생각하는 숫자를 입력하세요.");

        /*
        * String 타입의 인스턴스 생성하는 new 메서드 반환값
        * 변경이 가능한 변수를 선언하고 빈 값을 가진 String 타입의 새로운 인스턴스 할당
        */
        let mut guess = String::new();

        /*
        * & 기호는 해당 인자가 참조 타입임을 명시
        * 즉 다른 곳에서도 해당 데이터를 여러 번 메모리에 복사할 필요 없이 접근할 수 있다
        */
        io::stdin().read_line(&mut guess)
        /*
        * Result 타입을 통해 에러 처리
        * Error or OK (enum) 
        */
            .expect("입력한 값을 읽지 못했습니다.");

        // 기존 guess 변수가 보관하던 값을 overwrite 한다.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("숫자를 입력하세요"); continue;},
        };

        println!("입력한 값: {}", guess);

        match guess.cmp(&rand_num) {
            Ordering::Less => println!("입력한 숫자가 작습니다!"),
            Ordering::Greater => println!("입력한 숫자가 큽니다!"),
            Ordering::Equal => {println!("정답!"); break;}
        }
    }
}
