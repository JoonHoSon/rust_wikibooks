pub struct Node {
    data: i64,
    link: Option<Box<Node>>,
}

impl Node {
    fn new(data: i64, link: Option<Box<Node>>) -> Self {
        return Node { data, link };
    }
}

fn node(v: i64, link: Option<Box<Node>>) -> Option<Box<Node>> {
    return Some(Box::new(Node::new(v, link)));
}

fn main() {
    let c = node(10, node(20, node(30, None))).unwrap();
    let mut p = &c;

    loop {
        println!("{:#?}", p.data);

        match p.link {
            Some(ref link) => p = &link,
            None => break,
        }
    }
}
