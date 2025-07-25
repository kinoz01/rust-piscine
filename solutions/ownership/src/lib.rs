pub fn first_subword(s: String) -> String {
    let mut res = String::new();
    for (i, char) in s.chars().enumerate() {
        if (char.is_ascii_uppercase() && i != 0) || char == '_' {
            break;
        }
        res.push(char);
    }
    res
}