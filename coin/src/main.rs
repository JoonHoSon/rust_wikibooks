// 500원 : 10개
// 100원 : 3개
// 50원 : 10개

fn main() {
    let price = 3_950;

    for i500 in 0..11 {
        for i100 in 0..4 {
            for i50 in 0..11 {
                let total = i500 * 500 + i100 * 100 + i50 * 50;

                if total == price {
                    println!("500원 X {} + 100원 X {} + 50원 X {} = {}", i500, i100, i50, total);
                }
            }
        }
    }
}
