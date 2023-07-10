#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: std::ops::AddAssign,
{
    fn new(x: T, y: T) -> Self {
        return Self { x, y };
    }

    fn add(&mut self, pt: Point<T>) {
        self.x += pt.x;
        self.y += pt.y;
    }
}

fn main() {
    let mut point: Point<i32> = Point::new(10, 10);

    println!("{:?}", point);

    point.add(Point::new(20, 30));
    println!("{:?}", point);
}
