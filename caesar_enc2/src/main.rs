fn main() {
    let enc = encrypt("I LOVE RUST(사랑해요 RUST).", 3);
    let dec = encrypt(&enc, -3);

    println!("{} => {}", enc, dec);
}

fn encrypt(text: &str, shift: i16) -> String {
    let a = 'A' as u32;
    let is_az = |c| 'A' <= c && 'Z' >= c;
    let conv  = |c| {
        if shift.is_positive() {
            return (26 + c + a + (shift.abs() as u32)) % 26 + a;
        } else {
            return (26 + c + a - (shift.abs() as u32)) % 26 + a;
        }
    };
    let enc1 = |c| if is_az(c) { conv(c as u32) } else { c as u32 };

    text.chars().map(|c| char::from_u32(enc1(c)).unwrap()).collect()
}
