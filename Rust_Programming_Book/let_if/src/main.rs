fn main() {
    let x = 1.0;
    let y = 10;

    let result = if x < (y as f64) {
        "x는 y보다 작음"
    } else if x == (y as f64) {
        "x는 y와 같음"
    } else {
        "x는 y보다 큼"
    };

    // f64로 같음. 각 분기에서 할당되는 값들이 모두 동일한 타입이어야 함

    println!("{}", result);
}
