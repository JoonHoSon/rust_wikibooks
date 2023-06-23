extern crate rand;

use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    let range = 1..=5;

    println!("start {:?}", range.start());
    println!("end {:?}", range.end());

    for _ in 1..=5 {
        let dice = rng.gen_range(1..=6);

        println!("{:?}", dice);
    }
}
