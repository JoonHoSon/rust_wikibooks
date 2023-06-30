use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::process::exit;

fn main() {
    let filename = "fizzbuzz_file_result.txt";

    {
        let fp: File = match File::create(filename) {
            Ok(v) => v,
            Err(e) => {
                println!("{:?}", e.to_string());

                exit(1);
            }
        };

        let mut writer: BufWriter<File> = BufWriter::new(fp);

        for i in 1..=100 {
            let mut line = format!("{}\n", i);

            if i % 3 == 0 && i % 5 == 0 {
                line = String::from("FizzBuzz\n");
            } else if i % 3 == 0 {
                line = String::from("Fizz\n");
            } else if i % 5 == 0 {
                line = String::from("Buzz\n");
            }

            let bytes: &[u8] = line.as_bytes();

            match writer.write(bytes) {
                Ok(v) => v,
                Err(e) => {
                    println!("{:?}", e.to_string());

                    exit(1);
                }
            };
        }
    }

    let s: String = match fs::read_to_string(filename) {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);

            exit(1);
        }
    };
    println!("{}", s);
}
