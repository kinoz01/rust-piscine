pub fn count_factorial_steps(mut n: u64) -> u64 {
    if n < 2 {return 0}
    let mut c = 2;
    while n % c == 0 {
        n /= c;
        c += 1;
    }
    if n == 1 {c-1} else {0}
}