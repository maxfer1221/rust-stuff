// Tuples group together values of different types
// Max of 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Max", "Argentina", 19);
    println!("{} is from {} and is {}", person.0, person.1, person.2);

}
