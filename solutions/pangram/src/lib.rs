use std::collections::HashSet;
pub fn is_pangram(s: &str) -> bool {
    let res: HashSet<u8> = s
        .bytes()
        .filter(|&c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    res.len() == 26
}
