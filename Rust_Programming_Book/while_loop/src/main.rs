fn main() {
    let mut x = 0;
    while x < 5 {
        print!("{},", x);
        // 증감 연산자 X -> x++ 같은 코드 불가
        x += 1;
    }
    println!();
    let mut y = 0;
    let y = loop {
        y += 1;
        if y == 7 {
            break y; // y를 리턴하고 싶을 경우
        }
        print!("{},", y);
    };
}
