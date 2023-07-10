mod random {
    pub mod linear {
        use std::num::Wrapping;

        pub fn rand(seed: &mut usize) -> usize {
            let (a, c) = (234134345usize, 33248483942usize);

            *seed = (Wrapping::<usize>(*seed) * Wrapping::<usize>(a) + Wrapping::<usize>(c)).0;

            return *seed;
        }
    }

    pub mod xorshift {
        pub fn rand(seed: &mut usize) -> usize {
            // let mut y = *seed;

            // y ^= y << 13;
            // y ^= y >> 17;
            // y ^= y << 5;

            // *seed = y;

            // return y;

            *seed ^= *seed << 13;
            *seed ^= *seed >> 17;
            *seed ^= *seed >> 5;

            return *seed;
        }
    }
}

use random::{linear, xorshift};

fn main() {
    let mut seed1 = 12345usize;
    let mut seed2 = 12345usize;

    for i in 1..=10 {
        let r1 = linear::rand(&mut seed1) % 6 + 1;
        let r2 = xorshift::rand(&mut seed2) % 6 + 1;

        println!("L : {:2} 번째 = {}, {}", i, r1, r2);
    }
}
