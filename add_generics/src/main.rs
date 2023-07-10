fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    return a + b;
}

fn twice<F>(a: F) -> F
where
    F: std::ops::Add<Output = F> + Copy,
{
    return a + a;
}

fn main() {
    println!("{}", add(10, 25));
    println!("{}", add(10.0, 25.0));
    println!("{}", add::<i32>(10, 25));

    println!("{}", twice(10));
    println!("{}", twice(10.0));
    println!("{}", twice::<i32>(10));
}
