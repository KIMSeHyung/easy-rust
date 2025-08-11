use std::{mem::size_of};

fn give_number() -> i32 {
    // return 37 or
    37
}

fn main() {
    println!("Hello, world!");
    let my_number: i8 = 1;
    let second_number: i16 = 10;

    let third_number = my_number + second_number as i8; // type casting
    println!("{}", third_number);

    println!("size of char: {}", size_of::<char>());
    println!("size of string 'abc': {}", "abc".len());
    println!("size of string 'abc': {}", "가나다".len()); // byte 수
    println!("length of string 'abc': {}", "abc".chars().count());

    println!("my age is {}", give_number());
}