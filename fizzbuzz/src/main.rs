const FIZZ: &str = "Fizz";
const BUZZ: &str = "Buzz";
const FIZZ_BUZZ: &str = "FizzBuzz";

fn main() {
    for i in 1..=100 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("{}", FIZZ_BUZZ);
        } else if i % 3 == 0 {
            println!("{}", FIZZ);
        } else if i % 5 == 0 {
            println!("{}", BUZZ);
        } else {
            println!("{}", i);
        }
    }
}
