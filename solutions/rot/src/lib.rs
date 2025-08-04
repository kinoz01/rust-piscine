pub fn rotate(input: &str, key: i8) -> String {
    let k = ((key % 26) + 26) % 26;

    input
        .chars()
        .map(|c| match c {
            'a'..='z' => ((((c as u8) - b'a' + k as u8) % 26) + b'a') as char,
            'A'..='Z' => ((((c as u8) - b'A' + k as u8) % 26) + b'A') as char,
            _ => c,
        })
        .collect()
}