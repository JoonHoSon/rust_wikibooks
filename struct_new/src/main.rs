#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: String, age: i32) -> Self {
        return Person { name, age };
    }
}

fn main() {
    let yang: Person = Person::new("양현".to_string(), 18);

    println!("{}씨는 마음만은 {}살", yang.name, yang.age);
}
