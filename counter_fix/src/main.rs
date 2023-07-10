struct Counter {
    value: isize,
}

impl Counter {
    fn new() -> Self {
        return Counter { value: 0 };
    }

    fn inc(&mut self) {
        self.value += 1;
        println!("value = {}", self.value);
    }
}

fn count(counter: Option<&mut Counter>) {
    match counter {
        Some(cnt) => cnt.inc(),
        None => return,
    }
}

fn main() {
    let mut cnt = Counter::new();

    count(Some(&mut cnt));
    count(Some(&mut cnt));

    let a = None;

    count(a);
}
