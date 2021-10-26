mod random;
mod bubble;
mod selection;
mod binary;

fn main() {
    // let vec: Vec<u32> = random::random_array(100);

    // println!("Initial array: {:?}", vec);

    // Sort!
    // let bubble_sorted: Vec<u32> = bubble::sort(&vec);
    // let selection_sorted: Vec<u32> = selection::sort(&vec);
    //
    // println!("{:?}", bubble_sorted);
    // println!("{:?}", selection_sorted);

    // Binary search!
    let to_be_searched: Vec<u32> = (0..100).collect();

    let num = 43;
    println!(
        "Element {} found at index {}",
        num,
        binary::search(&to_be_searched, num, 0, to_be_searched.len()-1)
    );
}
