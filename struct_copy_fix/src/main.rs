#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: &str, age: i32) -> Self {
        return Person {
            name: name.to_string(),
            age: age,
        };
    }
}

fn main() {
    let alex: Person = Person::new("Alex", 18);
    let betty = Person {
        name: "Betty".to_string(),
        ..alex
    };

    // let mut betty = alex.clone();
    // betty.name = "Betty".to_string()

    println!("{}, {}", alex.name, alex.age);
    println!("{}, {}", betty.name, betty.age);
}
