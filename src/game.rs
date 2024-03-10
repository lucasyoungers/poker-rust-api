use core::fmt;
use std::cmp::Ordering;

use itertools::Itertools;

use crate::card::Card;
use crate::hand::Hand;
use crate::deck::Deck;
use crate::utils::combinations;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Player {
    name: String,
    hole_cards: [Card; 2]
}

#[allow(dead_code)]
impl Player {
    pub fn new(name: String, deck: &mut Deck) -> Self {
        let mut hole_cards: [Card; 2] = Default::default();
        for (i, card) in deck.deal(2).iter().enumerate() {
            hole_cards[i] = *card
        }
        Player { name, hole_cards }
    }

    pub fn hand(&self, community_cards: &Vec<Card>) -> Hand {
        if community_cards.len() < 3 {
            panic!("Can't form a hand with fewer than 3 community cards.");
        }

        let mut cards: Vec<Card> = Vec::new();
        cards.extend_from_slice(&self.hole_cards);
        cards.extend_from_slice(community_cards);

        let mut hands: Vec<Hand> = combinations(&cards, 5).iter().map(|c| {
            let mut cards_arr: [Card; 5] = Default::default();
            for (i, card) in c.iter().enumerate() {
                cards_arr[i] = *card;
            }
            Hand::new(&mut cards_arr)
        }).collect_vec();
        hands.sort_by(Hand::compare);

        hands[hands.len() - 1]
    }

    pub fn compare(a: &Self, b: &Self, community_cards: &Vec<Card>) -> Ordering {
        a.hand(community_cards).cmp(&b.hand(community_cards))
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.hole_cards.map(|c| c.to_string()).join(", "))
    }
}

#[allow(dead_code)]
pub struct Game {
    deck: Deck,
    players: Vec<Player>,
    community_cards: Vec<Card>
}

#[allow(dead_code)]
impl Game {
    pub fn new(names: &Vec<&str>) -> Self {
        let mut deck = Deck::new().shuffled();
        let players: Vec<Player> = names.iter().map(|s| Player::new(s.to_string(), &mut deck)).collect_vec();
        let community_cards: Vec<Card> = Vec::new();
        Game { deck, players, community_cards }
    }

    pub fn flop(&mut self) {
        self.deck.deal_to(&mut self.community_cards, 3);
    }

    pub fn turn(&mut self) {
        self.deck.deal_to(&mut self.community_cards, 1);
    }

    pub fn river(&mut self) {
        self.deck.deal_to(&mut self.community_cards, 1);
    }

    pub fn winner(&mut self) -> Player {
        let mut players: Vec<Player> = self.players.clone();
        players.sort_by(|a: &Player, b: &Player| Player::compare(a, b, &self.community_cards));
        players[players.len() - 1].clone()
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        if self.community_cards.len() > 0 {
            s += &format!("Community Cards: {}\n", self.community_cards.iter().map(|c| c.to_string()).join(", "));
        }
        s += &self.players.iter().map(|p| p.to_string()).join("\n");
        write!(f, "{}", s)
    }
}
