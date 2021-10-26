// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];

    numbers[2] = 20;

    // Print whole array
    println!("{:?}", numbers);

    // Print single value
    println!("single value: {}", numbers[2]);

    // Get array length
    println!("{}", numbers.len());

    // Arrays are stack allocated in bytes
    println!("{}", mem::size_of_val(&numbers));

    println!("{}", mem::size_of_val(&numbers[0]));

    // Get slice
    //let slice: &[i32] = &numbers[0..6];
    let slice: &[i32] = &numbers;
    println!("Slice {:?}", slice);

}
