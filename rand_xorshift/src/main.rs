use std::time::{SystemTime, UNIX_EPOCH};

fn init_rand() -> u32 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u32;
}

fn rand(seed: &mut u32, start: u32, end: u32) -> u32 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;

    return *seed % (end - start + 1) + start;
}

fn main() {
    let mut seed: u32 = init_rand();

    for i in 0..100 {
        let v = rand(&mut seed, 1, 6);

        if i > 0 && i % 10 == 0 {
            println!("");
        }

        print!("{}", v);
    }
}
