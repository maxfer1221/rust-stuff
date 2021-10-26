use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let num = input.trim().parse::<u64>().expect("That's not a number");

    triangle(num);
    triangle_b(num);
}

fn triangle_b(n: u64){
    for i in 0..n {
        for _j in 0..i+1 {
            print!("*");
        }
        println!();
    }
}

fn triangle(n: u64) {
    for i in 0..n {
        for _j in 0..(n-i) {
            print!("*");
        }
        println!();
    }
}
