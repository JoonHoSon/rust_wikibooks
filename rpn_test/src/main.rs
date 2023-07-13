extern crate rpn_calc_joonhoson;

use rpn_calc_joonhoson as rpn_calc;

fn main() {
    let source: String = "2 3 4 * +".to_string();
    // let answer: f64 = rpn_calc::eval(source).unwrap();

    // println!("answer : {}", answer);

    match rpn_calc::eval(source) {
        Ok(v) => println!("answer : {}", v),
        Err(e) => println!("{}", e),
    }
}
