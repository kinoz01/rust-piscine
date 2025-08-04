pub fn spell(n: u64) -> String {
    const ONES: [&str; 20] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
        "seventeen", "eighteen", "nineteen",
    ];
    const TENS: [&str; 10] = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    if n == 1_000_000 {
        return "one million".to_string();
    }

    fn helper(n: u64) -> String {
        if n < 20 {
            ONES[n as usize].to_string()
        } else if n < 100 {
            let t = (n / 10) as usize;
            let r = n % 10;
            if r == 0 {
                TENS[t].to_string()
            } else {
                format!("{}-{}", TENS[t], ONES[r as usize])
            }
        } else if n < 1_000 {
            let h = n / 100;
            let r = n % 100;
            if r == 0 {
                format!("{} hundred", helper(h))
            } else {
                format!("{} hundred {}", helper(h), helper(r))
            }
        } else {
            let th = n / 1_000;
            let r  = n % 1_000;
            if r == 0 {
                format!("{} thousand", helper(th))
            } else {
                format!("{} thousand {}", helper(th), helper(r))
            }
        }
    }

    helper(n)
}
