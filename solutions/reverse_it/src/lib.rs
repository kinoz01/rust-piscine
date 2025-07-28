pub fn reverse_it(v: i32) -> String {
    let abs_val = (v as i64).abs().to_string();
    let reversed: String = abs_val.chars().rev().collect();
    let sign = if v < 0 { "-" } else { "" };
    format!("{}{}{}", sign, reversed, abs_val)
}