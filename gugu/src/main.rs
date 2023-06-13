fn main() {
    for i in 1..10 {
        for j in 1..10 {
            print!("{:3},", i * j);
        }

        println!();
    }
}
