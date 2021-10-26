use std::io;

enum Op {
    Add,
    Subtract,
    Multiply,
    Divide
}

fn main() {
    println!("Welcome to the calculator.");
    loop {
        println!("What operation would you like to compute?");
        println!("(+) Addition");
        println!("(-) Subtraction");
        println!("(*) Multiplication");
        println!("(/) Division");
        let mut op = String::new();
        io::stdin().read_line(&mut op).expect("").trim();
        // let op = match io::stdin().read_line() {
        //     Ok() => {
        //
        //     }
        //     Err(_) => continue
        //}
    }
}
