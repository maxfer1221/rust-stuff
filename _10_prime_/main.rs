fn main() {
    let num: u32 = 150;

    let is_prime = | n: u32 | -> bool {
        for i in 2..(n as f64).sqrt() as u32 {
             if n % i == 0 { return false } } true };
             
    println!("{}", is_prime(num));

}
