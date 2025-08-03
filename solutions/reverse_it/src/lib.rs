pub fn reverse_it(v: i32) -> String {
    let res: String = (v as i64).abs().to_string().chars().rev().collect();
    let sign = if v < 0 {"-"} else {""};
    format!("{}{}{}", sign, res, (v as i64).abs())
}