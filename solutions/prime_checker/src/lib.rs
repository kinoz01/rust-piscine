#[derive(PartialEq, Eq, Debug)]
pub enum PrimeErr {
    Even,
    Divider(u32),
}

pub fn prime_checker(nb: u32) -> Option<Result<u32, PrimeErr>> {
    if nb <= 1 { return None; }
    if nb == 2 { return Some(Ok(2)); }
    if nb % 2 == 0 { return Some(Err(PrimeErr::Even)); }

    let mut d = 3u32;
    while d <= nb / d {
        if nb % d == 0 {
            return Some(Err(PrimeErr::Divider(d)));
        }
        d += 2;
    }
    Some(Ok(nb))
}
