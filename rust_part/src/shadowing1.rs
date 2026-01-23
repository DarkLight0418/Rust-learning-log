pub fn shadowing1() {
    let x = "5";

    let x = 6; // x를 6으로 재선언

    println!("The value of x is: {}", x); // 6
}
