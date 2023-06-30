use std::{env, path, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("find [path] [keyword]");

        return;
    }

    let target_dir = &args[1];
    let keyword = &args[2];
    let target = path::PathBuf::from(target_dir);

    findfile(&target, keyword);
}

fn findfile(target: &path::PathBuf, keyword: &str) {
    let files = match target.read_dir() {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    };

    for dir_entry in files {
        let path = dir_entry.unwrap().path();

        if path.is_dir() {
            findfile(&path, keyword);

            continue;
        }

        let filename = path.file_name().unwrap().to_string_lossy();

        if filename.find(keyword) == None {
            continue;
        }

        println!("{}", path.to_string_lossy());
    }
}
