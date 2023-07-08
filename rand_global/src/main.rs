use std::{
    process::exit,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

static mut SEED: u32 = 0;

unsafe fn rand_global(start: u32, end: u32) -> u32 {
    if SEED == 0 {
        let epoc: Duration = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(v) => v,
            Err(e) => {
                println!("{:?}", e);

                exit(1);
            }
        };

        SEED = epoc.as_millis() as u32;
    }

    SEED ^= SEED << 13;
    SEED ^= SEED >> 17;
    SEED ^= SEED << 5;

    return SEED % (end - start + 1) + start;
}

fn main() {
    for _ in 0..100 {
        unsafe {
            let v = rand_global(1, 6);

            println!("{}", v);
        }
    }
}
