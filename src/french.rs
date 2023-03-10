//! French deck implementation - classic 52 playing cards
use crate::deck::{Card as CardTrait, Deck as DeckTrait};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Copy, Debug)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Clone, Copy, Debug)]
pub enum Card {
    King(Suit),
    Queen(Suit),
    Jack(Suit),
    Pip(Suit, usize),
}

impl CardTrait for Card {
    fn string(&self) -> String {
        String::from("Card")
    }
}

pub struct FrenchDeck {
    cards: Vec<Card>,
}

impl FrenchDeck {
    pub fn new_unshuffled() -> FrenchDeck {
        let mut cards = Vec::new();
        for suit in [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade] {
            cards.push(Card::King(suit));
            cards.push(Card::Queen(suit));
            cards.push(Card::Jack(suit));
            for rank in 1..11 {
                cards.push(Card::Pip(suit, rank))
            }
        }
        FrenchDeck{ cards }
    }

    pub fn new() -> FrenchDeck {
        let mut deck = FrenchDeck::new_unshuffled();
        deck.shuffle();
        deck
    }
}

impl DeckTrait for FrenchDeck {
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn draw(&mut self) -> Option<Box<dyn CardTrait>> {
        match self.cards.pop() {
            Some(card) => Some(Box::new(card)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        FrenchDeck::new();
    }
    #[test]
    fn draw() {
        let mut deck = FrenchDeck::new();
        deck.draw();
        deck.draw();
    }
}
