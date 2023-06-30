use std::fs::{self, File};
use std::io::Write;
use std::process::exit;

fn main() {
    let filename: &str = "fizzbuzz_file2_result.txt";
    let result: String = get_fizzbuzz(100);

    {
        let mut fp = match File::create(filename) {
            Ok(f) => f,
            Err(e) => {
                println!("{:?}", e);

                exit(1);
            }
        };
        let bytes: &[u8] = result.as_bytes();

        match fp.write_all(bytes) {
            Ok(r) => r,
            Err(e) => {
                println!("{:?}", e);

                exit(1);
            }
        };
    }

    let s = match fs::read_to_string(filename) {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);

            exit(1);
        }
    };

    println!("{}", s);
}

fn get_fizzbuzz(max: u32) -> String {
    let mut result = String::new();

    for i in 1..=max {
        if i % 3 == 0 && i % 5 == 0 {
            result += "FizzBuzz\n";
        } else if i % 3 == 0 {
            result += "Fizz\n";
        } else if i % 5 == 0 {
            result += "Buzz\n";
        } else {
            result += &format!("{}\n", i);
        }
    }

    return result;
}
