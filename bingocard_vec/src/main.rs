extern crate rand;

use std::vec;

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};

fn main() {
    let mut nums = vec![];

    for i in 1..=75 {
        nums.push(i);
    }

    // shuffle
    let mut rng: ThreadRng = thread_rng();

    nums.shuffle(&mut rng);

    for i in 0..25 {
        if i == 12 {
            print!("  *,");
        } else {
            print!("{:3},", nums[i]);
        }

        if i % 5 == 4 {
            println!("");
        }
    }
}
