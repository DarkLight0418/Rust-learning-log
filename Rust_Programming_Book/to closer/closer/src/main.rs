fn temp() {
    // let my_func = |x| x + 1; // 클로저 생성
    let my_func1 = |x: i32| -> i32 { x + 1 }; // 위의 주석과 동일한 동작

    println!("{}", my_func1(3)); // 클로저 사용을 통한 변수 타입 유추(i32)
}

fn main() {
    let my_func = |mut x: i32| {
        x = x + 1;
        println!("{}", x);
    };

    my_func(3);
}
