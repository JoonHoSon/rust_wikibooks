fn main() {
    let price = 980_000.0;
    let a_ship_fee = 12_000.0;
    let a_discount_rate = 0.8;
    let b_ship_fee = 0.0;
    let b_discount_rate = 0.9;

    println!("A 쇼핑몰 : {}원", price * a_discount_rate + a_ship_fee);
    println!("B 쇼핑몰 : {}원", price * b_discount_rate + b_ship_fee);
}
