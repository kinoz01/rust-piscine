pub fn count_factorial_steps(factorial: u64) -> u64 {
    let mut n: u64 = factorial;
    if n < 2 {return 0};
    let mut c = 2;
    while n%c == 0 {
        n /= c;
        c +=1;
    }
    if n == 1 {c-1} else {0}
}