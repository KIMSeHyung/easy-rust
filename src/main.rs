use std::mem::size_of;
fn main() {
    println!("Hello, world!");
    let my_number: i8 = 1;
    let second_number: i16 = 10;

    let third_number = my_number + second_number as i8; // type casting
    println!("{}", third_number);

    println!("size of char: {}", size_of::<char>());
}