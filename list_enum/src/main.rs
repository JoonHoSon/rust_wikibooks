use Node::{Cons, Empty};

enum Node {
    Empty,
    Cons(i64, Box<Node>),
}

fn node(v: i64, link: Box<Node>) -> Box<Node> {
    return Box::new(Cons(v, link));
}

fn main() {
    let c = node(10, node(20, node(30, Box::new(Empty))));
    let mut ptr: &Box<Node> = &c;

    loop {
        let cur_node: &Node = &**ptr;

        match cur_node {
            Cons(v, link) => {
                println!("{:#?}", v);
                ptr = &link;
            }
            Empty => break,
        }
    }
}
