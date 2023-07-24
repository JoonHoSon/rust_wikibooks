use peg;

use crate::node::Node;

peg::parser!(pub grammar tomato() for str {
    // 기본 규칙 정의
    pub rule parse() -> Vec<Node> = v:sentences()

    // 프로그램에서 복문을 쓸 수 있도록 한다.
    rule sentences() -> Vec<Node> = sentence() ** end_of_line()

    // 구문 정의
    rule sentence() -> Node
        = print()
        / if()
        / for()
        / let()
        / _ {
            Node::Nop
        }

    // print 문 정의
    rule print() -> Node
        = "print" _ "\"" v:$([^ '"']*) "\"" {
            Node::PrintStr(v.to_string())
        }
        / "print" _ v:calc() {
            Node::Print(Box::new(v))
        }

    // if 문 정의
    rule if() -> Node
        = "if" _ v:if_condition() {
            v
        }

    rule if_condition() -> Node = if_elif() / if_else() / if_true_only()

    rule if_elif() -> Node
        = condition:calc() true_:block() lf() "elif" _ false_: if_condition() {
            Node::if_(condition, true_, vec![false_])
        }

    rule if_else() -> Node
        = condition:calc() true_:block() lf() "else" _ false_:block() {
            Node::if_(condition, true_, false_)
        }

    rule if_true_only() -> Node
        = condition:calc() true_:block() {
            Node::if_(condition, true_, vec![])
        }

    rule block() -> Vec<Node>
        = "{" _ v:sentences() _ "}" _ {
            v
        }

    // for 문 정의
    rule for() -> Node
        = "for" _ w:word() _ "=" _ start:number() _ "to" _ end:number() _ body:block() {
            println!("enter for loop");
            Node::For(w, start, end, Box::new(body))
        }

    // 변수 대입 정의
    rule let() -> Node
        = w:word() _ "=" _ v:calc() {
            Node::SetVar(w, Box::new(v))
        }

    // 계산 처리
    rule calc() -> Node = compare()

    rule compare() -> Node
        = l:expr() "==" _ r:compare() {
            Node::calc('=', l, r)
        }

        / l:expr() "!=" _ r:compare() {
            Node::calc('!', l, r)
        }

        / l:expr() ">" _ r:compare() {
            Node::calc('>', l, r)
        }

        / l:expr() ">=" _ r:compare() {
            Node::calc('g', l, r)
        }

        / l:expr() "<" _ r:compare()  {
            Node::calc('<', l, r)
        }

        / l:expr() "<=" _ r:compare() {
            Node::calc('l', l, r)
        }

        / expr()

    rule expr() -> Node
        = l:term() "+" _ r:calc() {
            Node::calc('+', l, r)
        }

        / l:term() "-" _ r:calc() {
            Node::calc('-', l, r)
        }

        / term()

    rule term() -> Node
        = l:val() "*" _ r:term() {
            Node::calc('*', l, r)
        }

        / l:val() "/" _ r:term() {
            Node::calc('/', l, r)
        }

        / l:val() "%" _ r:term() {
            Node::calc('%', l, r)
        }

        / val()

    rule val() -> Node
        = "(" _ v:calc() _ ")" _ {
            v
        }

        / v:number() _ {
            Node::Number(v)
        }

        / v:word() _ {
            Node::GetVar(v)
        }

    rule number() -> i64 = v:$(['0'..='9']+) {
        v.parse().unwrap()
    }

    // 변수명 정의
    rule word() -> String = v:$(['a'..='z' | 'A'..='Z' | '_']+['0'..='9']*) {
        String::from(v)
    }

    // 문장 나누기
    rule end_of_line() = [';' | '\n']+ _

    // 줄바꿈
    rule lf() = _ ['\n']* _

    // 공백 문자
    rule _ = [' ' | '\t']*
});
