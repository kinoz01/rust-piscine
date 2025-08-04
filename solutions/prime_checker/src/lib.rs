#[derive(PartialEq, Eq, Debug)]
pub enum PrimeErr {
    Even,
    Divider(u32),
}

pub fn prime_checker(nb: u32) -> Option<Result<u32, PrimeErr>> {
    match nb {
        0 | 1 => return None,
        2 => return Some(Ok(2)),
        _ if nb%2==0 => return Some(Err(PrimeErr::Even)),
        _ => {}
    }
    let mut d = 3u32;
    while d*d<= nb {
        if nb %d == 0 {
            return Some(Err(PrimeErr::Divider(d)))
        }
        d+=2
    }
    Some(Ok(nb))
}
