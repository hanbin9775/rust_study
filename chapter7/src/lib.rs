mod front_of_house;

// use 키워드는 파일 시스템에서 심볼릭 링크를 생성하는 것과 유사.
// hosting 모듈을 크레이트 루트에 정의한 것처럼 그 범위에서 유효한 이름이 된다.
pub use crate::front_of_house::hosting;

// pub use 키워드 덕분에 외부의 코드도 eat_at_restaurant를 호출할 수 있다.
pub fn eat_at_restaurant() {
    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();
    // 상대 경로
    front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// 모듈을 이용하면 관련된 정의들을 하나의 그룹으로 묶어 적절한 이름을 부여할 수 있다.

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super 키워드를 이용해 back_of_house 모듈의 부모 즉 루트 모듈인 crate에 접근한다.
        super::serve_order();
    }
    fn cook_order() {}

    pub struct Breakfast {
        // 구조체의 일부만 공개 가능
        pub toast: String,
        seasonal_fruit: String,
    }

    // 열거자는 모두 공개해야 의미가 있기 때문에 모두 공개
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// use 키워드에 상대 경로를 지정할 때는 self 키워드 사용
// use self::front_of_house::hosting;

// 함수 자체를 가져오는 방법은 함수를 범위로 가져오는 관용적인 방법이 아니다.
// 함수 경로의 반복을 최소화하면서도 이 함수가 로컬에 정의된 것이 아니라는 점을 더 명확히 할 수 있다.
// use crate::front_of_house::hosting::add_to_waitlist;

// re-exporting
// use 키워드를 이용해 범위로 이름을 가져오면 이 이름은 새 범위에서 비공개 이름이 된다.
// 우리가 작성한 코드를 호출하는 코드가 다른 범위에서 가져온 이름도 현재 범위에 정의된 것처럼 접근할 수 있도록 하는 기법.
// pub use crate::front_of_house::hosting;
