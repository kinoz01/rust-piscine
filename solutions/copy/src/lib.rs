pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c.abs() as f64).ln())
}

pub fn str_function(a: String) -> (String, String) {
    let exp_str = a.split_whitespace()
        .map(|s| (s.parse::<f64>().unwrap()).exp().to_string())
        .collect::<Vec<_>>()
        .join(" ");
    (a, exp_str)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let res = b
        .iter()
        .map(|&x| (x as f64).ln())
        .collect::<Vec<_>>();
    (b, res)
}