use std::print;

fn main() {
    let pr: &str = "구슬이 서 말이라도 꿰어야 보배";

    for (idx, c) in pr.bytes().enumerate() {
        print!("{:2x}", c);

        if idx < pr.bytes().len() - 1 {
            print!(" ");
        }
    }

    println!("\n바이트 = {}Bytes", pr.len());
}
