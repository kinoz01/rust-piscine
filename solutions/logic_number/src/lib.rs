pub fn number_logic(mut num: u32) -> bool {
    let mut digits = Vec::new();
    let mut temp = num;

    while temp > 0 {
        digits.push(temp % 10);
        temp /= 10;
    }

    let pow = digits.len() as u32;
    let sum: u32 = digits
        .iter()
        .map(|&d| d.pow(pow))
        .sum();

    sum == num
}
