// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure
// Use when you will modify the string data

pub fn run(){
    let mut hello = String::from("Hello ");

    println!("{}", hello.len());

    // Push character
    hello.push('W');

    // Push string
    hello.push_str("orld!");

    println!("{}\n{}", hello.len(), hello);

    // Capacity in bytes
    println!("{}", hello.capacity());

    // Is empty
    println!("{}", hello.is_empty());

    // Contains
    println!("{}", hello.contains("World"));

    // Replace
    println!("{}", hello.replace("World", "There"));

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
}
