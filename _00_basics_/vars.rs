// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped lanugae

pub fn run() {
    let name = "Max";
    let mut age = 19;

    println!("My name is {}, and I am {} years old", name, age);

    age = 20;

    println!("I am now {}", age);

    // Define constant
    const ID: i32 = 001;

    println!("ID: {}", ID);

    // Assign multiple variables at once
    let ( my_name, my_age ) = ( "Max", 19 );
    println!("{} is {}", my_name, my_age);
}
