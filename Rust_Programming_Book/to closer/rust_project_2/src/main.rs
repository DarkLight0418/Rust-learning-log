use crate::swap::swap;

mod add;
mod swap;

const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn do_nothing() -> () {
    return ();
}

fn me_too() {}

fn hello(name: String) {
    let num = 3;
    println!("Hello {}", name);
}

fn main() {
    let my_name = "melon".to_string();
    {
        println!("My name is {}", my_name);
        let my_name = "navy"; // 함수 내에서 not use
    }

    /// 변수 스코프 함수 범위 확인할 것 - 중괄호 기준으로 범위 바뀜
    hello(my_name);

    println!("{}", THRESHOLD);
    println!("{}", is_big(5));
    println!("{}", add::add(1, 2));
    let (num1, num2) = swap(1, 2);
    println!("{num1}, {num2}");

    println!("{:?}", do_nothing());
    println!("{:?}", me_too());
}
