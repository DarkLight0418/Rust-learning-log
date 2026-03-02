fn check_password(password: i32) -> bool {
    if password == 1234 {
        true
    } else {
        false
    }
}

fn main() {
    let password = 1234;
    let result = check_password(password);
    println!("Result: {}", result);

    for i in 6..10 {
        print!("{},", i);
    }
    println!();
    let num_range = 6..10;
    for i in num_range {
        print!("{},", i);
    }
}
