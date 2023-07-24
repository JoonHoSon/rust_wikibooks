peg::parser!(grammar calc() for str {
    pub rule eval() -> i64 = expr()

    // 덧셈
    rule expr() -> i64
        = l:term() "+" r:expr() {
            l + r
        }

        / l:term() "-" r:expr() {
            l - r
        }

        / term()

    rule term() -> i64
        = l:value() "*" r:term() {
            l * r
        }

        / l:value() "/" r:term() {
            l / r
        }

        / v:value()

    // 값을 읽는 규칙 추가
    rule value() -> i64
        = number()

        / "(" v:expr() ")" {
            v
        }

    // 숫자 값을 읽는 규칙 추가
    rule number() -> i64 = n:$(['0'..='9']+) {
        n.parse().unwrap()
    }
});

fn main() {
    println!("1 + 2 * 3 = {}", calc::eval("1+2*3").unwrap());
    println!("(1 + 2) * 3 = {}", calc::eval("(1+2)*3").unwrap());
    println!("100 / 2 - 1 = {}", calc::eval("100/2-1").unwrap());
}
