fn main() {
    let x = 1.0;
    let y = 10;

    if x < (y as f64) {
        // 타입 캐스팅
        println!("x is less than y");
    } else if x == (y as f64) {
        println!("x is equal to y");
    } else {
        println!("x is not less than y");
    }
}
