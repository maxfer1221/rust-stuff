fn main() {
    println!("{}", change(0));
    println!("{}", change(12));
    println!("{}", change(468));
    println!("{}", change(123456));
}

fn change(mut t: i32) -> i32 {
    let mut change: i32 = 0;
    let mut vec: Vec<i32> = vec![500, 100, 25, 10, 5, 1];
    while t > 0 {
        if vec[0] > t {
            vec.remove(0);
        } else {
            t -= vec[0];
            change += 1;
        }
    }
    change
}
