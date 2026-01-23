use std::env::var;

mod shadowing1;
mod var1;

fn main() {
    println!("Please run 'rustfmt!'");
    var1::var1();
    shadowing1::shadowing1();
}
