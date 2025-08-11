fn main() {
    let args: Vec<String> = std::env::args().collect();

    rpn(&args[1]);
}

pub fn rpn(input: &str) {
    let mut res: Vec<i64> = Vec::new();
    let mut err = false;

    for v in input.split_whitespace() {
        if let Ok(x) = v.parse() {
            res.push(x);
        } else {
            if (v == "+" || v == "-" || v == "*" || v == "/" || v == "%") && res.len() < 2 {
                err = true;
                break;
            }
            let (y, x) = (res.pop().unwrap(), res.pop().unwrap());
            match v {
                "+" => res.push(x + y),
                "-" => res.push(x - y),
                "/" => res.push(x / y),
                "*" => res.push(x * y),
                "%" => res.push(x % y),
                _ => {
                    err = true;
                    break;
                }
            }
        }
    }
    if res.len() == 1 && !err {
        println!("{}", res[0])
    } else {
        println!("Error")
    }
}
