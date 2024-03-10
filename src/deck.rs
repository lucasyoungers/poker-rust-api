use core::fmt;

use crate::card::Card;
use crate::card::RANKS;
use crate::card::SUITS;
use crate::utils::shuffle;
use crate::utils::shuffle_mut;

#[allow(dead_code)]
pub struct Deck { cards: Vec<Card> }

#[allow(dead_code)]
impl Deck {
    pub fn new() -> Self {
        let mut cards: Vec<Card> = vec![];
        for s in SUITS {
            for r in RANKS {
                let chars: Vec<char> = vec![r, s];
                let string: String = chars.iter().collect();
                let card: Card = Card::parse(&string).unwrap();
                cards.push(card);
            }
        }
        Deck { cards }
    }

    pub fn from(cards: Vec<Card>) -> Self {
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        shuffle_mut(&mut self.cards)
    }

    pub fn shuffled(&self) -> Self {
        Deck::from(shuffle(&self.cards))
    }

    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn remove(&mut self, card: Card) {
        for i in 0..self.cards.len() {
            if self.cards[i] == card {
                self.cards.remove(i);
            }
        }
    }
    
    pub fn deal(&mut self, n: usize) -> Vec<Card> {
        if n > self.cards.len() {
            panic!("Can't deal more cards than are in the deck.");
        }
        let len = self.cards.len();
        let cards = self.cards[len-n..len].to_vec();
        self.cards.truncate(len-n);
        cards
    }

    pub fn deal_to(&mut self, out: &mut Vec<Card>, n: usize) {
        out.extend(self.deal(n));
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cards.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", "))
    }
}
