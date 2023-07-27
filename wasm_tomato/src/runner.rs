use std::collections::HashMap;

use crate::node::Node;
use crate::parser::tomato;

/// 프로그램 전체에서 사용할 컨텍스트 정의
struct Context {
    /// 변수와 값 저장
    vars: HashMap<String, i64>,
    output: String,
}

impl Context {
    fn new() -> Self {
        Context {
            vars: HashMap::new(),
            output: String::new(),
        }
    }
}

fn calc_op(op: char, val_l: i64, val_r: i64) -> i64 {
    match op {
        '+' => val_l + val_r,
        '-' => val_l - val_r,
        '*' => val_l * val_r,
        '/' => val_l / val_r,
        '%' => val_l % val_r,
        '=' => {
            if val_l == val_r {
                1
            } else {
                0
            }
        }
        '!' => {
            if val_l != val_r {
                1
            } else {
                0
            }
        }
        '>' => {
            if val_l > val_r {
                1
            } else {
                0
            }
        }
        'g' => {
            if val_l >= val_r {
                1
            } else {
                0
            }
        }
        '<' => {
            if val_l < val_r {
                1
            } else {
                0
            }
        }
        'l' => {
            if val_l <= val_r {
                1
            } else {
                0
            }
        }
        _ => 0,
    }
}

fn run_nodes(ctx: &mut Context, nodes: &Vec<Node>) -> i64 {
    let mut result = 0;

    nodes.iter().for_each(|node| {
        result = run_node(ctx, node.clone());
    });

    result
}

/// 구문 트리를 하나씩 실행
fn run_node(ctx: &mut Context, node: Node) -> i64 {
    match node {
        // 숫자 값을 반환
        Node::Number(v) => v,

        // 계산식
        Node::Calc(op, l, r) => calc_op(op, run_node(ctx, *l), run_node(ctx, *r)),

        // 숫자 값 얻기
        Node::GetVar(name) => match ctx.vars.get(&name) {
            Some(v) => *v,
            None => 0,
        },

        // 변수 대입
        Node::SetVar(name, node) => {
            let val = run_node(ctx, *node);

            ctx.vars.insert(name, val);

            val
        }

        // If 문
        Node::If(condition, true_, false_) => {
            let condition_value = run_node(ctx, *condition);

            if condition_value > 0 {
                run_nodes(ctx, &*true_)
            } else {
                run_nodes(ctx, &*false_)
            }
        }

        // For 문
        Node::For(name, start, end, body) => {
            let mut r = 0;
            let nodes = *body;

            for i in start..=end {
                ctx.vars.insert(name.clone(), i);

                r = run_nodes(ctx, &nodes);
            }

            r
        }

        // 문자열 print
        Node::PrintStr(v) => {
            ctx.output += &format!("{}\n", v);

            0
        }

        // 숫자 print
        Node::Print(node) => {
            let v = run_node(ctx, *node);
            ctx.output += &format!("\n{}", v);

            v
        }

        _ => 0,
    }
}

pub fn run(src: &str) -> String {
    let nodes = match tomato::parse(src) {
        Ok(res) => res,
        Err(e) => return e.to_string(),
    };
    let mut ctx = Context::new();
    let r = run_nodes(&mut ctx, &nodes);

    if ctx.output == "" {
        return format!("{}", r);
    } else {
        return ctx.output.clone();
    }
}
