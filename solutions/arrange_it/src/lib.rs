pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();
    words.sort_by_key(|w| w.chars().find(|c| c.is_ascii_digit()).unwrap_or('0'));
    words
        .iter()
        .map(|w| w.chars().filter(|c| !c.is_ascii_digit()).collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}