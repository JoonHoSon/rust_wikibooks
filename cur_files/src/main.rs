use std::{
    fs::{self, DirEntry, ReadDir},
    process::exit,
};

fn main() {
    let files: ReadDir = match fs::read_dir(".") {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    };

    for f in files {
        let entry: DirEntry = match f {
            Ok(v) => v,
            Err(e) => {
                println!("{:?}", e);
                exit(1);
            }
        };
        let path = entry.path();
        let filename = path.to_str().unwrap_or("올바르지 않은 파일 이름 입니다.");

        println!("{}", filename);
    }
}
