use std::cmp::{ Ord, Ordering };
use std::fmt::{ self, Debug };
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for Antigen {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err("invalid antigen".into()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err("invalid rh".into()),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.antigen, &self.rh_factor).cmp(&(&other.antigen, &other.rh_factor))
    }
}

impl FromStr for BloodType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rh = s.chars().last().ok_or("too short")?;
        let ag = &s[..s.len() - 1];
        Ok(BloodType {
            antigen: ag.parse()?,
            rh_factor: rh.to_string().parse()?,
        })
    }
}

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ag = match self.antigen {
            Antigen::A => "A",
            Antigen::AB => "AB",
            Antigen::B => "B",
            Antigen::O => "O",
        };
        let rh = if matches!(self.rh_factor, RhFactor::Positive) { "+" } else { "-" };
        write!(f, "{}{}", ag, rh)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        // Rh: negative can't receive positive; others OK
        let rh_ok = match &self.rh_factor {
            RhFactor::Negative =>
                match &other.rh_factor {
                    RhFactor::Positive => false,
                    _ => true,
                }
            _ => true,
        };

        // ABO
        let abo_ok = match (&self.antigen, &other.antigen) {
            (Antigen::AB, _) => true,
            (Antigen::A, Antigen::A) | (Antigen::A, Antigen::O) => true,
            (Antigen::B, Antigen::B) | (Antigen::B, Antigen::O) => true,
            (Antigen::O, Antigen::O) => true,
            _ => false,
        };

        rh_ok && abo_ok
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut v: Vec<_> = all_types()
            .into_iter()
            .filter(|t| self.can_receive_from(t))
            .collect();
        v.sort();
        v
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut v: Vec<_> = all_types()
            .into_iter()
            .filter(|t| t.can_receive_from(self))
            .collect();
        v.sort();
        v
    }
}

fn all_types() -> Vec<BloodType> {
    use Antigen::*;
    use RhFactor::*;
    vec![
        BloodType { antigen: A, rh_factor: Positive },
        BloodType { antigen: A, rh_factor: Negative },
        BloodType { antigen: AB, rh_factor: Positive },
        BloodType { antigen: AB, rh_factor: Negative },
        BloodType { antigen: B, rh_factor: Positive },
        BloodType { antigen: B, rh_factor: Negative },
        BloodType { antigen: O, rh_factor: Positive },
        BloodType { antigen: O, rh_factor: Negative }
    ]
}
