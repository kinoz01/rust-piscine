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
            if c.is_lowercase() {
                c.to_uppercase().collect::<Vec<_>>()
            } else {
                c.to_lowercase().collect::<Vec<_>>()
            }
        })
        .collect()
}
