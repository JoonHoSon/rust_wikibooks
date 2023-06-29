use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    // index 1 : 사전파일(dict.txt)
    // index 2 : 찾고자 하는 단어

    if args.len() < 3 {
        println!("사용 방법은 ./dictionary [경로를 포함하는 사전파일] [찾고자 하는 단어]");

        exit(1);
    }

    let dictionary: &String = &args[1];
    let word: &String = &args[2];
    let fp: File = match File::open(dictionary) {
        Ok(p) => p,
        Err(e) => {
            println!("{:?}", e.to_string());

            exit(1);
        }
    };
    let reader: BufReader<File> = BufReader::new(fp);

    for line in reader.lines() {
        let line = line.unwrap();

        if line.find(word) == None {
            continue;
        }

        println!("{}", line);
    }
}
