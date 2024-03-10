use core::fmt;
use std::cmp::Ordering;

#[allow(dead_code)]
pub const RANKS: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

#[allow(dead_code)]
pub const SUITS: [char; 4] = ['S', 'H', 'C', 'D'];

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Rank {
    #[default]
    Default,
    Value(u8)
}

#[allow(dead_code)]
impl Rank {
    pub fn new(value: u8) -> Option<Self> {
        if value >= 2 && value <= 14 {
            Some(Rank::Value(value))
        } else {
            None
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            Rank::Default => 0,
            Rank::Value(v) => *v
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Rank::Default => '0',
            Rank::Value(2) | Rank::Value(3) | Rank::Value(4) |
            Rank::Value(5) | Rank::Value(6) | Rank::Value(7) |
            Rank::Value(8) | Rank::Value(9) => (self.value() + b'0') as char,
            Rank::Value(10) => 'T',
            Rank::Value(11) => 'J',
            Rank::Value(12) => 'Q',
            Rank::Value(13) => 'K',
            Rank::Value(14) => 'A',
            _ => panic!("Should never reach here.")
        };
        write!(f, "{}", c)
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Suit {
    #[default]
    Default,
    S, H, C, D
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Suit::Default => '0',
            Suit::S => 'S',
            Suit::H => 'H',
            Suit::C => 'C',
            Suit::D => 'D'
        };
        write!(f, "{}", c)
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Card { rank: Rank, suit: Suit }

#[allow(dead_code)]
impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card { rank, suit }
    }

    pub fn value(&self) -> u8 {
        self.rank.value()
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn parse(string: &str) -> Option<Self> {
        if string.len() != 2 {
            return None;
        }

        let rank_char: char = string.chars().nth(0).unwrap();
        let rank_value: Option<u8> = match rank_char {
            '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => rank_char.to_digit(10).map(|x| x as u8),
            'T' => Some(10),
            'J' => Some(11),
            'Q' => Some(12),
            'K' => Some(13),
            'A' => Some(14),
            _ => None
        };
        
        if let None = rank_value {
            return None;
        }

        let rank: Option<Rank> = Rank::new(rank_value.unwrap());

        let suit_char: char = string.chars().nth(1).unwrap();
        let suit: Option<Suit> = match suit_char {
            'S' => Some(Suit::S),
            'H' => Some(Suit::H),
            'C' => Some(Suit::C),
            'D' => Some(Suit::D),
            _ => None
        };

        match (rank, suit) {
            (Some(r), Some(s)) => Some(Card::new(r, s)),
            _ => None
        }
    }

    pub fn compare(a: &Self, b: &Self) -> Ordering {
        a.value().cmp(&b.value())
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}
