use std::env;

fn main() {
    for arg in env::args().skip(1) {
        println!("{}", if is_bracketed_ok(&arg) { "OK" } else { "Error" });
    }
}

fn is_bracketed_ok(s: &str) -> bool {
    let mut stack = Vec::new();
    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' | ']' | '}' => {
                match stack.pop() {
                    Some('(') if ch == ')' => {}
                    Some('[') if ch == ']' => {}
                    Some('{') if ch == '}' => {}
                    _ => {
                        return false;
                    }
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}
