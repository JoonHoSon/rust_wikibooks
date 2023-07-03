fn main() {
    let mut v = 16;
    x2(&mut v);
    println!("{}", v);
}

fn x2(arg: &mut i32) {
    *arg = *arg * 2;
}
