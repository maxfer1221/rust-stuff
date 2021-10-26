// Loops - Used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    // Infinite loops
    loop {
        count += 1;
        println!("Number: {}", count);
        if count == 20 {
            break;
        }
    }

    // While loop
    let mut i = 0;
    while i <= 100 {
        if i % 15 == 0 {
            println!("FizzBuzz");
        }
        else if i % 5 == 0 {
            println!("Buzz");
        }
        else if i % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", i);
        }
        i+=1;
    }

    // For Range
    for j in 0..100 {
        if j % 15 == 0 {
            println!("FizzBuzz");
        }
        else if j % 5 == 0 {
            println!("Buzz");
        }
        else if j % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", j);
        }
    }
}
