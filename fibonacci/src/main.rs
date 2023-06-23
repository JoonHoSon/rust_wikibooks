fn main() {
    let mut a = 1;
    let mut b = 1;

    println!("{}", a);
    println!("{}", b);

    for _ in 0..=29 {
        println!("{}", a + b);

        let temp = a;
        a = b;
        b = temp + b;
    }
}
