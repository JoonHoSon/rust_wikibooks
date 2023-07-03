use std::{
    env,
    fs::{Metadata, Permissions},
    os::unix::prelude::PermissionsExt,
    path,
    process::exit,
};

extern crate colored;

use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut target_dir = ".";

    if args.len() > 1 {
        target_dir = &args[1];
    }

    let target = path::PathBuf::from(target_dir);
    println!("{}", target_dir);

    tree(&target, 0);
}

fn tree(target: &path::PathBuf, level: isize) {
    let files: std::fs::ReadDir = match target.read_dir() {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    };

    for f in files {
        let path: path::PathBuf = match f {
            Ok(v) => v.path(),
            Err(e) => {
                println!("{:?}", e);
                exit(1);
            }
        };

        for _ in 1..=level {
            print!("|    ");
        }

        let filename = match path.file_name() {
            Some(v) => v.to_string_lossy(),
            None => {
                println!("파일명을 확인할 수 없습니다.");
                exit(1);
            }
        };

        if path.is_dir() {
            println!("|---- <{}>", filename);
            tree(&path, level + 1);

            continue;
        }

        let metadata: Metadata = match path.metadata() {
            Ok(v) => v,
            Err(e) => {
                println!("{:?}", e);
                exit(1);
            }
        };
        let permission: Permissions = metadata.permissions();
        let executable: bool = permission.mode() & 0o111 != 0;

        if executable {
            println!("|---- {}", filename.cyan());
        } else {
            println!("|---- {}", filename);
        }
    }
}
