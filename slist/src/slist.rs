pub struct Node {
    data: isize,
    link: Option<Box<Node>>,
}

impl Node {
    pub fn new(data: isize, link: Option<Box<Node>>) -> Self {
        return Self { data, link };
    }
}

pub struct List {
    head: Option<Box<Node>>,
}

impl List {
    pub fn new() -> Self {
        return Self { head: None };
    }

    pub fn unshift(&mut self, v: isize) {
        let new_node = Node::new(v, self.head.take());
        self.head = Some(Box::new(new_node));
    }

    pub fn push(&mut self, v: isize) {
        let new_node = Node::new(v, None);

        match self.head {
            None => self.head = Some(Box::new(new_node)),
            Some(ref mut head) => {
                let mut p = head;

                loop {
                    match p.link {
                        None => {
                            p.link = Some(Box::new(new_node));
                            break;
                        }
                        Some(ref mut next) => p = next,
                    }
                }
            }
        }
    }

    pub fn get(&self, index: isize) -> Option<isize> {
        match self.head {
            None => None,
            Some(ref top) => {
                let mut p = top;
                let mut i = 0;

                loop {
                    if i == index {
                        return Some(p.data);
                    }

                    match p.link {
                        None => None,
                        Some(ref link) => Some(p = link),
                    };

                    i += 1;
                }
            }
        }
    }
}
