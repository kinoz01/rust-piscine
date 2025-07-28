pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

pub fn title_case(input: &str) -> String {
    input
        .split_inclusive(|c: char| c.is_whitespace())
        .map(capitalize_first)
        .collect()
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .flat_map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().collect::<Vec<char>>()
            } else {
                c.to_uppercase().collect::<Vec<char>>()
            }
        })
        .collect()
}
