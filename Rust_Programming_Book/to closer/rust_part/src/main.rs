use std::env::var;

mod change_type;
mod shadowing1;
mod type1;
mod var1;

fn main() {
    println!("Please run 'rustfmt!'");
    var1::var1();
    shadowing1::shadowing1();
    type1::type1();
    change_type::change_type();
}
