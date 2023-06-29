use std::{
    env::{self, Args},
    fs,
};

fn main() {
    let args: Args = env::args();
    let mut total = 0.0f64;

    for (i, filename) in args.enumerate() {
        if i == 0 {
            continue;
        }

        let text: String = match fs::read_to_string(filename) {
            Ok(v) => v,
            Err(_) => "0.0".to_string(),
        };

        for v in text.split("\n") {
            total += match v.parse::<f64>() {
                Ok(v) => v,
                Err(_) => 0.0,
            }
        }
    }

    println!("total : {}", total);
}
