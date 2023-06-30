fn main() {
    for i in 1..=20 {
        println!("{:?}", fib(i));
    }
}

fn fib(n: i64) -> i64 {
    if n == 1 {
        return 0;
    }

    if n == 2 {
        return 1;
    }

    fib(n - 2) + fib(n - 1)
}
