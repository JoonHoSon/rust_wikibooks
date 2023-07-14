extern crate image;

use std::{path::PathBuf, process::exit};

use image::{GenericImage, GenericImageView, Rgba};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("[USAGE] image_filter imagefile");

        return;
    }

    let working_dir: PathBuf = match std::env::current_dir() {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);

            exit(1);
        }
    };
    let mut temp_dir: PathBuf = match std::env::current_exe() {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    };

    temp_dir.pop();

    let execution_dir: PathBuf = if working_dir != temp_dir {
        working_dir
    } else {
        temp_dir
    };

    let file_path: PathBuf = PathBuf::from(&args[1]);
    let full_file_path: PathBuf = if file_path.is_relative() {
        execution_dir.as_path().join(file_path)
    } else {
        file_path
    };

    if !full_file_path.exists() {
        println!("이미지를 찾을 수 없습니다.\n{:?}", full_file_path);

        exit(1);
    }

    let filename: Vec<&str> = full_file_path
        .to_str()
        .unwrap()
        .split(std::path::MAIN_SEPARATOR_STR)
        .collect();
    let filename: Vec<&str> = filename.last().unwrap().split(".").collect();

    let outfile: String = format!("{}-out.jpg", filename[0]);
    let mut img = image::open(full_file_path).expect("파일을 읽어올 수 없습니다.");
    let (w, h) = img.dimensions();

    for y in 0..h {
        for x in 0..w {
            let c: Rgba<u8> = img.get_pixel(x, y);
            let c = Rgba([255 - c[0], 255 - c[1], 255 - c[2], c[3]]);

            img.put_pixel(x, y, c);
        }
    }

    img.save(outfile).unwrap();
}
