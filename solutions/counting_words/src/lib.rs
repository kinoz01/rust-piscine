use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    let cc: String = words.to_ascii_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '\'' {c} else {' '})
        .collect();

    for mut w in cc.split_whitespace() {
        w = w.trim_matches('\'');
        if w.is_empty() {continue}
        *map.entry(w.to_string()).or_insert(0) += 1;
    }
    map
}