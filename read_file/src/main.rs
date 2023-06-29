use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("읽어올 파일을 지정해 주세요");
        exit(1);
    }

    let filename: &String = &args[1];
    let text: String = match fs::read_to_string(filename) {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e.to_string());
            exit(1);
        }
    };

    println!("{}", text);
}
