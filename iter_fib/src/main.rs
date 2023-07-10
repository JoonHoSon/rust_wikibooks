struct FiboIteraotr {
    a: usize,
    b: usize,
}

impl FiboIteraotr {
    fn new() -> Self {
        return FiboIteraotr { a: 1, b: 1 };
    }
}

impl Iterator for FiboIteraotr {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let temp = self.a;
        self.a = self.b;
        self.b += temp;

        return Some(self.a);
    }
}

fn main() {
    let fibo: FiboIteraotr = FiboIteraotr::new();

    for (i, n) in fibo.enumerate() {
        if i >= 10 {
            break;
        }

        print!("{}", n);
    }

    println!("");

    let fibo = FiboIteraotr::new();

    fibo.take(10).for_each(|f| print!("{}", f));

    println!("");
}
