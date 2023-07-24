#[derive(Debug, Clone)]
pub enum Node {
    /// 아무것도 하지 않음
    Nop,

    /// 숫자 값을 나타냄
    Number(i64),

    /// 계산식
    Calc(char, Box<Node>, Box<Node>),

    /// if 문
    If(Box<Node>, Box<Vec<Node>>, Box<Vec<Node>>),

    /// for 문
    For(String, i64, i64, Box<Vec<Node>>),

    /// print 문 (계산 출력)
    Print(Box<Node>),

    /// print 문 (상수 출력)
    PrintStr(String),

    /// 변수 대입
    SetVar(String, Box<Node>),

    /// 변수 참조
    GetVar(String),
}

impl Node {
    /// `Node::calc` 타입을 반환 하는 함수
    pub fn calc(op: char, left: Node, right: Node) -> Node {
        Node::Calc(op, Box::new(left), Box::new(right))
    }

    /// `Node::If` 타입을 반환 하는 함수
    pub fn if_(condition: Node, true_: Vec<Node>, false_: Vec<Node>) -> Node {
        println!("--- [if_] --- ");
        println!("condition : {:?}", condition);
        println!("true      : {:?}", true_);
        println!("false     : {:?}", false_);
        Node::If(Box::new(condition), Box::new(true_), Box::new(false_))
    }
}
