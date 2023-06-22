const CODE_A: u32 = 'A' as u32;
const CODE_Z: u32 = 'Z' as u32;

fn main() {
    let enc: String = encrypt("I LOVE YOU~~(사랑해요)", 3);
    let dec = encrypt(&enc, -3);

    println!("{} => {}", enc, dec);
}

fn encrypt(text: &str, shift: i16) -> String {
    let mut result = String::new();

    for chr in text.chars() {
        let mut code = chr as u32;

        if CODE_A <= code && CODE_Z >= code {
            if shift.is_positive() {
                code = (26 + code - CODE_A + (shift.abs() as u32)) % 26 + CODE_A;
            } else {
                code = (26 + code - CODE_A - (shift.abs() as u32)) % 26 + CODE_A;
                // 26을 더하는 구문이 괄호안에서 마지막에 위치할 경우
                // attempt to subtract with overflow 오류 발생
            }
        }

        result.push(char::from_u32(code).unwrap());
    }

    return result;
}
