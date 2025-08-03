mod edit_distance;

use convert_case::{ Case, Casing };
use edit_distance::*;

pub fn expected_variable(input: &str, expected: &str) -> Option<String> {
    let inp = input.trim();
    let exp = expected.trim();

    fn is_snake(s: &str) -> bool {
        s.contains('_') &&
            s.chars().all(|c| (c.is_ascii_alphanumeric() || c == '_')) &&
            s.to_ascii_lowercase() == s.to_case(Case::Snake)
    }

    fn is_camel(s: &str) -> bool {
        !s.contains('_') &&
            s.chars().all(|c| c.is_ascii_alphanumeric()) &&
            s.chars().any(char::is_uppercase) &&
            s.chars().any(char::is_lowercase)
    }

    if !(is_snake(inp) || is_camel(inp)) {
        return None;
    }

    let a = inp.to_ascii_lowercase();
    let b = exp.to_ascii_lowercase();

    let len = b.len().max(1) as f64;
    let dist = edit_distance(&a, &b) as f64;
    let similarity = ((1.0 - dist / len) * 100.0).round() as i32; // nearest %

    (similarity > 50).then(|| format!("{similarity}%"))
}
