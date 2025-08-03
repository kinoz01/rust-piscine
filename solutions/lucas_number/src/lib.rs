pub fn lucas_number(n: u32) -> u32 {
    match n {
        0 => 2,
        1 => 1,
        _ => {
            let (mut a, mut b) = (2, 1);
            for _ in 2..=n {
                let next = a + b;
                a = b;
                b = next;
            }
            b
        }
    }
}
