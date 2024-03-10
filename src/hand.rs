use core::fmt;
use std::cmp::Ordering;

use itertools::Itertools;

use crate::card::Card;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Hand { cards: [Card; 5], ranks: [u8; 15] }

#[allow(dead_code)]
impl Hand {
    pub fn new(cards: &mut [Card; 5]) -> Self {
        cards.sort_by(Card::compare);

        let mut ranks: [u8; 15] = [0; 15];
        for card in cards.iter() {
            ranks[card.value() as usize] += 1
        }

        Hand { cards: *cards, ranks }
    }

    pub fn high_card(&self) -> Card {
        self.cards[self.cards.len() - 1]
    }

    pub fn has_pair(&self) -> bool {
        self.ranks.iter().any(|&x| x == 2)
    }

    pub fn has_three_kind(&self) -> bool {
        self.ranks.iter().any(|&x| x == 3)
    }

    pub fn has_two_pair(&self) -> bool {
        self.ranks.iter().filter(|&&x| x == 2).collect::<Vec<&u8>>().len() == 2
    }

    pub fn is_straight(&self) -> bool {
        let mut prev: u8 = self.cards[0].value();
        for i in 1..self.cards.len() {
            let curr: u8 = self.cards[i].value();
            if prev == 5 && curr == 14 {
                return true;
            }
            if prev != curr - 1 {
                return false;
            }
            prev = curr;
        }
        true
    }

    pub fn is_flush(&self) -> bool {
        let suit = self.cards[0].suit();
        self.cards.iter().all(|&c| c.suit() == suit)
    }

    pub fn is_full_house(&self) -> bool {
        self.has_three_kind() && self.has_pair()
    }

    pub fn is_four_kind(&self) -> bool {
        self.ranks.iter().any(|&x| x == 4)
    }

    pub fn is_straight_flush(&self) -> bool {
        self.is_straight() && self.is_flush()
    }

    pub fn score(&self) -> u8 {
        if self.is_straight_flush() {
            9
        } else if self.is_four_kind() {
            8
        } else if self.is_full_house() {
            7
        } else if self.is_flush() {
            6
        } else if self.is_straight() {
            5
        } else if self.has_three_kind() {
            4
        } else if self.has_two_pair() {
            3
        } else if self.has_pair() {
            2
        } else {
            1
        }
    }

    pub fn compare(a: &Self, b: &Self) -> Ordering {
        if a.score() != b.score() {
            return a.score().cmp(&b.score());
        }

        match a.score() {
            9 => Self::compare_straight_flush(a, b),
            8 => Self::compare_four_kind(a, b),
            7 => Self::compare_full_house(a, b),
            6 => Self::compare_flush(a, b),
            5 => Self::compare_straight(a, b),
            4 => Self::compare_three_kind(a, b),
            3 => Self::compare_two_pair(a, b),
            2 => Self::compare_pair(a, b),
            _ => Self::compare_high_card(a, b)
        }
    }

    fn compare_high_card(a: &Self, b: &Self) -> Ordering {
        for i in a.cards.len()-1..=0 {
            let a_value = a.cards[i].value();
            let b_value = b.cards[i].value();
            if a_value != b_value {
                return a_value.cmp(&b_value);
            }
        }
        Ordering::Equal
    }

    fn compare_pair(a: &Self, b: &Self) -> Ordering {
        let a_rank = a.ranks.iter().position(|&x| x == 2).unwrap() as u8;
        let b_rank = b.ranks.iter().position(|&x| x == 2).unwrap() as u8;
        if a_rank == b_rank {
            let a_other = a.cards.iter().filter(|&&c| c.value() != a_rank).collect::<Vec<&Card>>();
            let b_other = b.cards.iter().filter(|&&c| c.value() != b_rank).collect::<Vec<&Card>>();
            for i in a_other.len()-1..=0 {
                let a_value = a_other[i].value();
                let b_value = b_other[i].value();
                if a_value != b_value {
                    return a_value.cmp(&b_value);
                }
            }
            return Ordering::Equal;
        }
        a_rank.cmp(&b_rank)
    }

    fn compare_two_pair(a: &Self, b: &Self) -> Ordering {
        let a_ranks = a.ranks.iter().positions(|&x| x == 2).collect::<Vec<usize>>();
        let a_other = a.ranks.iter().position(|&x| x == 1).unwrap();

        let b_ranks = b.ranks.iter().positions(|&x| x == 2).collect::<Vec<usize>>();
        let b_other = b.ranks.iter().position(|&x| x == 1).unwrap();

        if a_ranks[1] != b_ranks[1] {
            return a_ranks[1].cmp(&b_ranks[1]);
        }

        if a_ranks[0] != b_ranks[0] {
            return a_ranks[0].cmp(&b_ranks[0]);
        }

        a_other.cmp(&b_other)
    }

    fn compare_three_kind(a: &Self, b: &Self) -> Ordering {
        let a_rank = a.ranks.iter().position(|&x| x == 3).unwrap() as u8;
        let b_rank = b.ranks.iter().position(|&x| x == 3).unwrap() as u8;
        if a_rank == b_rank {
            let a_other = a.cards.iter().filter(|&&c| c.value() != a_rank).collect::<Vec<&Card>>();
            let b_other = b.cards.iter().filter(|&&c| c.value() != b_rank).collect::<Vec<&Card>>();
            for i in a_other.len()-1..=0 {
                let a_value = a_other[i].value();
                let b_value = b_other[i].value();
                if a_value != b_value {
                    return a_value.cmp(&b_value);
                }
            }
            return Ordering::Equal;
        }
        a_rank.cmp(&b_rank)
    }

    fn compare_straight(a: &Self, b: &Self) -> Ordering {
        let mut a_score = a.high_card().value();
        if a_score == 14 && a.cards[0].value() == 2 {
            a_score = 5;
        }

        let mut b_score = b.high_card().value();
        if b_score == 14 && b.cards[0].value() == 2 {
            b_score = 5;
        }

        a_score.cmp(&b_score)
    }

    fn compare_flush(a: &Self, b: &Self) -> Ordering {
        for i in a.cards.len()-1..=0 {
            let a_value = a.cards[i].value();
            let b_value = b.cards[i].value();
            if a_value != b_value {
                return a_value.cmp(&b_value);
            }
        }
        Ordering::Equal
    }

    fn compare_full_house(a: &Self, b: &Self) -> Ordering {
        let a_three_rank = a.ranks.iter().position(|&x| x == 3).unwrap();
        let b_three_rank = b.ranks.iter().position(|&x| x == 3).unwrap();
        if a_three_rank != b_three_rank {
            return a_three_rank.cmp(&b_three_rank);
        }

        let a_two_rank = a.ranks.iter().position(|&x| x == 2).unwrap();
        let b_two_rank = b.ranks.iter().position(|&x| x == 2).unwrap();
        a_two_rank.cmp(&b_two_rank)
    }

    fn compare_four_kind(a: &Self, b: &Self) -> Ordering {
        let a_four_rank = a.ranks.iter().position(|&x| x == 4).unwrap();
        let b_four_rank = b.ranks.iter().position(|&x| x == 4).unwrap();
        if a_four_rank != b_four_rank {
            return a_four_rank.cmp(&b_four_rank);
        }

        let a_one_rank = a.ranks.iter().position(|&x| x == 1).unwrap();
        let b_one_rank = b.ranks.iter().position(|&x| x == 1).unwrap();
        a_one_rank.cmp(&b_one_rank)
    }

    fn compare_straight_flush(a: &Self, b: &Self) -> Ordering {
        let mut a_score = a.high_card().value();
        if a_score == 14 && a.cards[0].value() == 2 {
            a_score = 5;
        }

        let mut b_score = b.high_card().value();
        if b_score == 14 && b.cards[0].value() == 2 {
            b_score = 5;
        }

        a_score.cmp(&b_score)
    }

    pub fn parse(string: &str) -> Option<Self> {
        let mut cards: [Card; 5] = Default::default();
        let strings = string.split(", ").collect::<Vec<&str>>();
        if strings.len() != 5 {
            return None;
        }
        for (i, &s) in strings.iter().enumerate() {
            match Card::parse(s) {
                Some(card) => cards[i] = card,
                None => return None
            };
        }
        Some(Hand::new(&mut cards))
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cards.map(|c| c.to_string()).join(", "))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::compare(self, other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        Self::compare(self, other)
    }
}
