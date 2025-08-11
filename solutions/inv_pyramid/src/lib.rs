pub fn inv_pyramid(st: String, max: usize) -> Vec<String> {
    let mut res = Vec::with_capacity(2*max);
    let mut add_line = |n| res.push(format!("{:>1$}", st.repeat(n), 2*n));
    for n in 1..=max {
        add_line(n);
    }
    for n in (1..max).rev() {
        add_line(n);
    }
    res
}
