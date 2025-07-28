pub fn inv_pyramid(st: String, max: usize) -> Vec<String> {
    let mut vec = Vec::with_capacity(2 * max);
    let mut add_line = |amount| vec.push(format!("{:>1$}", st.repeat(amount), 2 * amount));
    for n in 1..=max {
        add_line(n);
    }
    for n in (1..max).rev() {
        add_line(n);
    }
    vec
}