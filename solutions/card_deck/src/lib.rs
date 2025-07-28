use rand::random_range;

#[derive(Debug, PartialEq, Eq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl Suit {
    pub fn random() -> Self {
        let r = random_range(1..5);
        Self::translate(r)
    }

    pub fn translate(value: u8) -> Self {
        match value {
            1 => Self::Heart,
            2 => Self::Diamond,
            3 => Self::Spade,
            _ => Self::Club,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Rank {
    pub fn random() -> Self {
        let r = random_range(1..14);
        Self::translate(r)
    }

    pub fn translate(value: u8) -> Self {
        match value {
            1 => Self::Ace,
            2..=10 => Self::Number(value),
            11 => Self::Jack,
            12 => Self::Queen,
            _ => Self::King,
        }
    }
}

#[derive(PartialEq, Debug, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}
