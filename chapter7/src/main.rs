use std::fmt::Result;
// as 키워드로 새로운 이름을 부여
use std::io::Result as IoResult;

//use std::io;
//use std::cmp::Ordering;
// 중첩 경로를 이용해 같은 경로로부터 여러 아이템을 가져올 수 있다.
use std::{cmp::Ordering, io};

// self 키워드를 사용하여 구문을 합칠 수 있다.
// use std::io::{self, Write};

// 어떤 경로의 공개 아이템을 모두 현재 범위로 가져오려면 급롭 연산자인 * 을 사용하여 경로를 지정하면 된다.
use std::collections::*;

fn main() {
    println!("Hello, world!");
}
