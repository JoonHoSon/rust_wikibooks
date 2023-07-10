enum Currency {
    Currency100(isize),
    Currency500(isize),
    Currency1_000(isize),
    Currency5_000(isize),
    Currency10_000(isize),
    Currency50_000(isize),
}

impl Currency {
    fn calc_price(&self) -> isize {
        match self {
            Currency::Currency100(v) => v * 100,
            Currency::Currency500(v) => v * 500,
            Currency::Currency1_000(v) => v * 1_000,
            Currency::Currency5_000(v) => v * 5_000,
            Currency::Currency10_000(v) => v * 10_000,
            Currency::Currency50_000(v) => v * 50_000,
        }
    }
}

fn main() {
    let wallet: Vec<Currency> = vec![
        Currency::Currency100(3),
        Currency::Currency500(2),
        Currency::Currency1_000(6),
        Currency::Currency5_000(1),
        Currency::Currency10_000(8),
        Currency::Currency50_000(3),
    ];
    let total: isize = wallet.iter().fold(0, |sum, v| sum + v.calc_price()); // fold(init_value, closure)

    println!("지갑 안의 금액은 {}원 입니다", total);
}
