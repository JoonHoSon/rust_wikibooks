use std::process::exit;

fn main() {
    println!("내용을 입력하세요(3글자 이상).",);

    let mut s: String = String::new();

    std::io::stdin().read_line(&mut s).expect("입력 오류");

    // carriage return 포함
    if s.chars().count() < 4 {
        println!("3글자 이상을 입력하여 주세요.",);

        exit(1);
    }

    for (idx, chr) in s.chars().enumerate() {
        if chr == ' ' && idx == s.chars().count() - 1 {
            continue;
        }

        println!("{}", chr);
    }

    println!("1번째 글자 : {}", s.chars().nth(0).unwrap());
    println!("3번째 글자 : {}", s.chars().nth(2).unwrap());
}
