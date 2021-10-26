extern crate rand;

use rand::thread_rng;
use rand::Rng;

pub fn random_array(size: u32,) -> Vec<u32> {
    let mut vec: Vec<u32> = vec![];

    let mut rng = thread_rng();
    for _i in 0..size {
        vec.push(rng.gen_range(0,100));
    }

    vec
}
