extern crate rand;

use std::print;

use rand::{seq::SliceRandom, thread_rng};

fn main() {
    let mut nums = [0; 75];

    for i in 1..=75 {
        nums[i - 1] = i;
    }

    // shuffle
    let mut rng = thread_rng();

    nums.shuffle(&mut rng);

    for y in 0..=4 {
        for x in 0..=4 {
            let i = y * 5 + x;

            if i == 12 {
                // wildcard
                print!(" *,");
            } else {
                print!("{:3}, ", nums[i]);
            }
        }

        println!("");
    }
}
