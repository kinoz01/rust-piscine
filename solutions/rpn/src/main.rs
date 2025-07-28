use std::env;

fn main() {
    // expect exactly one argument after the program name
    let mut args = env::args().skip(1);
    match (args.next(), args.next()) {
        (Some(expr), None) => match eval_rpn(&expr) {
            Some(val) => println!("{}", val),
            None => println!("Error"),
        },
        _ => println!("Error"),
    }
}

/// Evaluate a Reverse-Polish-Notation expression.
/// Returns None on any syntax/arithmetical error.
fn eval_rpn(expr: &str) -> Option<i64> {
    let mut stack: Vec<i64> = Vec::new();

    for token in expr.split_whitespace() {
        match token {
            "+" | "-" | "*" | "/" | "%" => {
                let (b, a) = (stack.pop()?, stack.pop()?);
                let res = match token {
                    "+" => a.checked_add(b)?,
                    "-" => a.checked_sub(b)?,
                    "*" => a.checked_mul(b)?,
                    "/" => {
                        if b == 0 { return None }
                        a.checked_div(b)?
                    }
                    "%" => {
                        if b == 0 { return None }
                        a.checked_rem(b)?
                    }
                    _ => unreachable!(),
                };
                stack.push(res);
            }
            num => stack.push(num.parse::<i64>().ok()?),
        }
    }
    if stack.len() == 1 { Some(stack[0]) } else { None }
}
