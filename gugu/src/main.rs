fn main() {
    // 기본 출력 방식
    for i in 1..10 {
        for j in 1..10 {
            if j < 9 {
                print!("{:3},", i * j);
            } else {
                print!("{:3}", i * j);
            }
        }

        println!();
    }

    // 단의 제목 출력 후 2 ~ 5단 출력
    // 다음 문단에 6 ~ 9단 출력
    // 2, 6단을 제외 하고 이전 단에서 5자리 공백 유지
    // 2 x 9 = 18 ==> 총 10자리

    // 단 제목 출력
    for title in 2..6 {
        print_line(title);
    }

    println!();

    for title in 2..6 {
        print_title(title);
    }

    println!();

    for title in 2..6 {
        print_line(title);
    }

    println!();

    for second in 1..10 {
        for first in 2..6 {
            print_rule(first, second);
        }

        println!();
    }

    println!("\n");

    for title in 6..10 {
        print_line(title);
    }

    println!();

    for title in 6..10 {
        print_title(title);
    }

    println!();

    for title in 6..10 {
        print_line(title);
    }

    println!();

    for second in 1..10 {
        for first in 6..10 {
            print_rule(first, second);
        }

        println!();
    }
}

fn print_line(num: i32) {
    if num != 2 && num != 6 {
        print!("{}", " ".repeat(5));
    }

    print!("{}", "-".repeat(10));
}

fn print_title(num: i32) {
    if num != 2 && num != 6 {
        print!("{}", " ".repeat(5));
    }

    print!("{}", format!("{: ^10}", format!("{}", num)));
}

fn print_rule(first: i32, second: i32) {
    if first != 2 && first != 6 {
        print!("{}", " ".repeat(5));
    }

    print!(
        "{}",
        format!(
            "{:<10}",
            format!("{} x {} = {}", first, second, first * second)
        )
    );
}
