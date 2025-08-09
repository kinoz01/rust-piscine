pub fn negative_spell(n: i64) -> String {
    if n == 0 {
        return "zero".into();
    }
    if n > 0 {
        return "error: positive number".into();
    }

    let x = -n as u64;

    fn spell(n: u64) -> String {
        const ONES: [&str; 20] = [
            "zero",
            "one",
            "two",
            "three",
            "four",
            "five",
            "six",
            "seven",
            "eight",
            "nine",
            "ten",
            "eleven",
            "twelve",
            "thirteen",
            "fourteen",
            "fifteen",
            "sixteen",
            "seventeen",
            "eighteen",
            "nineteen",
        ];
        const TENS: [&str; 10] = [
            "",
            "",
            "twenty",
            "thirty",
            "forty",
            "fifty",
            "sixty",
            "seventy",
            "eighty",
            "ninety",
        ];

        match n {
            0..=19 => ONES[n as usize].into(),
            20..=99 => {
                let t = n / 10;
                let r = n % 10;
                if r == 0 {
                    TENS[t as usize].into()
                } else {
                    format!("{}-{}", TENS[t as usize], ONES[r as usize])
                }
            }
            100..=999 => {
                let h = n / 100;
                let r = n % 100;
                if r == 0 {
                    format!("{} hundred", ONES[h as usize])
                } else {
                    format!("{} hundred {}", ONES[h as usize], spell(r))
                }
            }
            1_000..=999_999 => {
                let th = n / 1_000;
                let r = n % 1_000;
                if r == 0 {
                    format!("{} thousand", spell(th))
                } else {
                    format!("{} thousand {}", spell(th), spell(r))
                }
            }
            1_000_000 => "one million".into(),
            _ => unreachable!(),
        }
    }

    format!("minus {}", spell(x))
}
