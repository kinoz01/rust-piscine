pub fn delete_and_backspace(s: &mut String) {
    let mut res = String::new();
    let mut skip = 0;
    
    for c in s.chars() {
        if skip > 0 && c != '+' {
            skip -= 1;
            continue;
        }
        match c {
            '+' => skip += 1,
            '-' => {res.pop();},
            _ => res.push(c),
        }
    }
    *s = res;
}

pub fn do_operations(v: &mut [String]) {
    for s in v.iter_mut() {
        let res = if let Some((a, b)) = s.split_once('+') {
            a.trim().parse::<i32>().unwrap() + b.trim().parse::<i32>().unwrap()
        } else if let Some((a, b)) = s.split_once('-') {
            a.trim().parse::<i32>().unwrap() - b.trim().parse::<i32>().unwrap()
        } else {
            continue;
        };
        *s = res.to_string();
    }
}