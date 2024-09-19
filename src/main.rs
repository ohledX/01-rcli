use std::borrow::Borrow;

fn main() {
    println!("Hello, world!");

    let x: i32 = 1;
    let y = x.borrow();
    println!("x: {}, y: {}", x, y);
}
