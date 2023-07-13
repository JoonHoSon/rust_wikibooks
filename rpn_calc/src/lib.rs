// 문서용 설명 작성
//! # RPN Calc
//! Reverse Polish notation (RPN) Calc.
//! # Example
//! ```
//! let source = "1 2 + 3 *".to_string();
//! let a = rpn_calc::eval(source).unwrap();
//! println!("{}", a); // 9
//! ```
pub fn eval(source: String) -> Result<f64, String> {
    let tokens = source.split_whitespace();
    let mut stack: Vec<f64> = vec![];

    for token in tokens {
        let t = token.trim();

        if t == "" {
            continue;
        }

        match t.parse::<f64>() {
            Ok(v) => {
                stack.push(v);

                continue;
            }
            Err(_) => 0.0,
        };

        let b = stack.pop().unwrap_or(0.0f64);
        let a = stack.pop().unwrap_or(0.0f64);

        match t {
            "+" => stack.push(a + b),
            "-" => stack.push(a - b),
            "*" => stack.push(a * b),
            "/" => stack.push(a / b),
            "%" => stack.push(a % b),
            _ => return Err(format!("Invalid operator: [{}]", t)),
        }
    }

    if stack.len() == 0 {
        return Err(format!("No result."));
    }

    if stack.len() > 1 {
        return Err(format!("Too many value in stack."));
    }

    return Ok(stack.pop().unwrap_or(0.0f64));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(eval("1 3 +".to_string()), Ok(4.0f64));
        assert_eq!(eval("2 3 *".to_string()), Ok(6.0f64));
        assert_eq!(eval("6 3 /".to_string()), Ok(2.0f64));
        assert_eq!(eval("6 3 - 1 -".to_string()), Ok(2.0f64));
    }
}
