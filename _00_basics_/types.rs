/*
primitive types---
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
Floats: f32, f64
Boolean: bool
Characters: char
Tuples
Ar rays
*/

// Rust is a statically typed language, must know the types of
// all variables at compile time
// Rust can infer types so we don't always need to declare types

pub fn run(){
    // Default is "i32"
    let _x = 2;

    // Default is "f64"
    let _y = 2.5;

    // Add explicit type
    let _z: i64 = 1000;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    println!("{:?}", (_x, _y, _z, is_active));

    // Get boolean from expression
    let is_even: bool = _x % 2 == 0;

    println!("{} even: {}", _x, is_even);

    let a1: char = 'a';
    println!("{}", a1);
}
