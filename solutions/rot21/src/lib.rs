pub fn rot21(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            'a'..='z' => (((c as u8 - b'a' + 21) % 26) + b'a') as char,
            'A'..='Z' => (((c as u8 - b'A' + 21) % 26) + b'A') as char,
            _ => c,
        })
        .collect()
}
