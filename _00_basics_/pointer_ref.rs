// Reference Pointers - Point to a resource in memory

use std::any;

pub fn run() {
    // Primitive Array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    // Non-primitives no longer get the value, but a pointer to it
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));

    const vec_test1: Vec<i32> = Vec::new();
    let mut vec_test2 = vec_test1;
    vec_test2.push(1);

    println!("{:?}",(vec_test1, vec_test2));
    /*
    allow
    */
}
