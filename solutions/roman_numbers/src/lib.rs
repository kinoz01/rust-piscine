use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit { Nulla, I, V, X, L, C, D, M }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(n: u32) -> Self {
        match n {
            0 => Nulla, 1 => I, 5 => V, 10 => X, 50 => L, 100 => C, 500 => D, 1000 => M,
            _ => Nulla,
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut n: u32) -> Self {
        if n == 0 { return RomanNumber(vec![Nulla]); }
        const MAP: &[(u32, &[RomanDigit])] = &[
            (1000, &[M]), (900, &[C, M]), (500, &[D]), (400, &[C, D]),
            (100, &[C]),  (90,  &[X, C]), (50,  &[L]), (40,  &[X, L]),
            (10,  &[X]),  (9,   &[I, X]), (5,   &[V]), (4,   &[I, V]),
            (1,   &[I]),
        ];
        let mut out = Vec::new();
        for &(v, digs) in MAP {
            while n >= v {
                out.extend_from_slice(digs);
                n -= v;
            }
        }
        RomanNumber(out)
    }
}
