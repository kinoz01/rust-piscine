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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        // Rh: negative can't receive positive; others OK
        let rh_ok = !(
            matches!(self.rh_factor, RhFactor::Negative) &&
            matches!(other.rh_factor, RhFactor::Positive)
        );

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
        Self::all_types()
            .into_iter()
            .filter(|d| self.can_receive_from(d))
            .collect()
    }

    pub fn recipients(&self) -> Vec<Self> {
        Self::all_types()
            .into_iter()
            .filter(|r| r.can_receive_from(self))
            .collect()
    }

    fn all_types() -> Vec<Self> {
        use Antigen::*;
        use RhFactor::*;
        vec![
            Self { antigen: AB, rh_factor: Positive },
            Self { antigen: O, rh_factor: Positive },
            Self { antigen: A, rh_factor: Positive },
            Self { antigen: B, rh_factor: Positive },
            Self { antigen: AB, rh_factor: Negative },
            Self { antigen: O, rh_factor: Negative },
            Self { antigen: A, rh_factor: Negative },
            Self { antigen: B, rh_factor: Negative }
        ]
    }
}
