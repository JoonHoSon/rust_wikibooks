extern crate image;

fn main() {
    let white: image::Rgb<u8> = image::Rgb::<u8>([255, 255, 255]);
    let red: image::Rgb<u8> = image::Rgb::<u8>([255, 0, 0]);
    let width: u32 = 64;

    let draw = |x, y| {
        let (xi, yi) = (x / width, y / width);

        match (xi % 2, yi % 2) {
            (0, 0) => white,
            (1, 0) => red,
            (0, 1) => red,
            (1, 1) => white,
            (_, _) => panic!("Error!!!"),
        }
    };

    let img = image::ImageBuffer::from_fn(512, 512, draw);

    img.save("checkerboard.png").unwrap();
}
