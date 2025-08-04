pub fn get_diamond(c: char) -> Vec<String> {
    let n = (c as u8 - b'A') as usize;
    let width = 2 * n + 1;

    (0..=n)
        .chain((0..n).rev())
        .map(|i| {
            let ch = (b'A' + i as u8) as char;
            if i == 0 {
                format!("{:^width$}", ch, width = width)
            } else {
                let inner = 2 * i - 1;
                let pair = format!("{}{}{}", ch, " ".repeat(inner), ch);
                format!("{:^width$}", pair, width = width)
            }
        })
        .collect()
}