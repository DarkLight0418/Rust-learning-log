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

fn main() {
    println!("{}", THRESHOLD);
    println!("{}", is_big(5));
    println!("{}", add::add(1, 2));
    let (num1, num2) = swap(1, 2);
    println!("{num1}, {num2}");

    println!("{:?}", do_nothing());
    println!("{:?}", me_too());
}
