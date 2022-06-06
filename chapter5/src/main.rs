struct User {
    username: String,
    active: bool,
}

// 튜플 구조체
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Debug 특성은 개발자에게 유용한 형식으로 구조체의 값을 출력하므로 디버깅하는 동안 구조체의 값을 확인하는데 유용하다.
#[derive(Debug)] // debug 특성 추가.
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 첫 번째 인자를 self 로만 선언해서 인스턴스에 대한 소유권을 갖는 메서드는 드물다. 예전 인스턴스에 대한 접근을 제한할 때 사용
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, target: &Rectangle) -> bool {
        self.width >= target.width && self.height >= self.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // 구조체를 생성하는 방법들
    let user1 = User {
        username: String::from("kim"),
        active: true,
    };

    let user2 = build_user(String::from("lee"));

    let user3 = User { ..user1 }; // 소유권 이전 user1 -> user3

    println!("{}, {}", user3.username, user2.username);

    // black 과 origin 은 다른 튜플 구조체의 인스턴스이기 때문에 다른 타입이다.
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}", area(&rect1));
    println!("rect1: {:#?}", rect1);

    println!("{}", rect1.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("{}", rect1.can_hold(&rect2));

    println!("{:?}", Rectangle::square(10))
}

fn build_user(username: String) -> User {
    User {
        username,
        active: true,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
