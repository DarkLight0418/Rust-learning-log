fn multiply_numbers(x: i32, y: i32) -> i32 {
    x * y
}

fn main() {
    let result = multiply_numbers(3, 4);
    println!("The product of 3 and 4 is: {}", result);
}

/*
틀린 답안...

fn multiply_numbers(let x:i32, let y:i32) {
    return x * y:i32;
}

fn main() {
    let result = multiply_numbers(3, 4);
    println!("The product of 3 and 4 is: {}", result);
}

*/
