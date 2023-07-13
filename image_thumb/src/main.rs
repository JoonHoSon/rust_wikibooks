extern crate image;

use image::{imageops, GenericImageView};

fn main() {
    let size: u32 = 300;
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("[USAGE] image_thumb imagefile");

        return;
    }

    let infile = &args[1].to_string();
    let filename: Vec<&str> = infile.split(".").collect();
    let outfile_nearset: String = format!("{}-thumb-nearest.png", filename[0]);
    let outfile_triangle: String = format!("{}-thumb-triangle.png", filename[0]);
    let outfile_catmulrom: String = format!("{}-thumb-catmulrom.png", filename[0]);
    let outfile_gaussian: String = format!("{}-thumb-gaussian.png", filename[0]);
    let outfile_lanczos3: String = format!("{}-thumb-lanczos3.png", filename[0]);
    let mut img = image::open(infile).expect("파일을 읽어올 수 없습니다.");
    let dim: (u32, u32) = img.dimensions();
    let width = if dim.0 > dim.1 { dim.1 } else { dim.0 };
    let mut img_resize = imageops::crop(
        &mut img,
        (dim.0 - width) / 2,
        (dim.1 - width) / 2,
        width,
        width,
    );
    let nearest_result: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        imageops::resize(&mut img_resize, size, size, imageops::Nearest);
    let triangle_result: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        imageops::resize(&mut img_resize, size, size, imageops::Triangle);
    let catmulrom_result: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        imageops::resize(&mut img_resize, size, size, imageops::CatmullRom);
    let gaussian_result: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        imageops::resize(&mut img_resize, size, size, imageops::Gaussian);
    let lanczos3_result: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        imageops::resize(&mut img_resize, size, size, imageops::Lanczos3);

    nearest_result.save(outfile_nearset).unwrap();
    triangle_result.save(outfile_triangle).unwrap();
    catmulrom_result.save(outfile_catmulrom).unwrap();
    gaussian_result.save(outfile_gaussian).unwrap();
    lanczos3_result.save(outfile_lanczos3).unwrap();
}
