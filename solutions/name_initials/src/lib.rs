pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res = Vec::with_capacity(names.len());
    for name in names {
        let mut init = String::new();
        
        for (i, word) in name.split_whitespace().enumerate() {
            if let Some(ch) = word.chars().next() {
                if i > 0 {
                    init.push(' ');
                }
                init.push(ch.to_ascii_uppercase());
                init.push('.');
            }
         }
        res.push(init);
    }
    res
}