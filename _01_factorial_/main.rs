// fn main() {
//     println!("{:?}", (
//         factorial(5),
//         factorial(10),
//         factorial(12),
//         factorial(6),
//     ));
// }
//
// fn factorial(mut n: u32) -> u32 {
//     let mut res: u32 = 1;
//     while n > 1 {
//         res *= n;
//         n -= 1;
//     }
//     (n > 0) as u32 * res + (n < 1) as u32
// }

fn main() {
    println!("{}", fun());
}

fn fun() -> i32 {
    let x: i32 = 3;
    {
        x
    };
    return x
}
