use std::fmt;

#[derive(Copy, Clone)]
pub enum SuitType {
    Club,
    Spade,
    Diamond,
    Heart
}

// write implementation display for suit
// write display for card symbols
impl fmt::Display for SuitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SuitType::Spade => write!(f, "Spade"),
            SuitType::Heart => write!(f, "Heart"),
            SuitType::Diamond => write!(f, "Diamond"),
            SuitType::Club => write!(f, "Club"),
        }
    }
}

pub enum CardSymbol {
    One(i32),
    Two(i32),
    Three(i32),
    Four(i32),
    Five(i32),
    Six(i32),
    Seven(i32),
    Eight(i32),
    Nine(i32),
    Ten(i32),
    Jack(i32),
    Queen(i32),
    King(i32),
    Ace(i32)
}

impl fmt::Display for CardSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CardSymbol::One(value) => write!(f, "one {}", value),
            CardSymbol::Two(value) => write!(f, "two {}", value),
            CardSymbol::Three(value) => write!(f, "three {}", value),
            CardSymbol::Four(value) => write!(f, "four {}", value),
            CardSymbol::Five(value) => write!(f, "five {}", value),
            CardSymbol::Six(value) => write!(f, "six {}", value),
            CardSymbol::Seven(value) => write!(f, "seven {}", value),
            CardSymbol::Eight(value) => write!(f, "eight {}", value),
            CardSymbol::Nine(value) => write!(f, "nine {}", value),
            CardSymbol::Ten(value) => write!(f, "ten {}", value),
            CardSymbol::Jack(value) => write!(f, "jack {}", value),
            CardSymbol::Queen(value) => write!(f, "queen {}", value),
            CardSymbol::King(value) => write!(f, "king {}", value),
            CardSymbol::Ace(value) => write!(f, "ace {}", value),
        }
    }
}



// how to represent card symbol?
// how to randomize cards
pub struct Card {
    // make suit a reference to suit?
    pub suit: i32,
    pub symbol: CardSymbol,
}