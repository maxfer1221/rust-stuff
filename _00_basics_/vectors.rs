// Vectors - resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];

    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    // Print whole vector
    println!("{:?}", numbers);

    // Print single value
    println!("single value: {}", numbers[2]);

    // Get vector length
    println!("{}", numbers.len());

    // Vectors are stack allocated in bytes
    println!("{}", mem::size_of_val(&numbers));

    println!("{}", mem::size_of_val(&numbers[0]));

    // Get slice
    //let slice: &[i32] = &numbers[0..6];
    let slice: &[i32] = &numbers;
    println!("Slice {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number x: {}", x);
    }

    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
}
