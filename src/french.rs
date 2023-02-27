use crate::deck::Deck;
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

pub struct FrenchDeck {
    cards: Vec<Card>,
}

impl FrenchDeck {
    pub fn new() -> FrenchDeck {
        let mut cards = Vec::new();
        for suit in [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade] {
            cards.push(Card::King(suit));
            cards.push(Card::Queen(suit));
            cards.push(Card::Jack(suit));
            for rank in 1..11 {
                cards.push(Card::Pip(suit, rank))
            }
        }
        let mut deck = FrenchDeck {
            cards: cards,
        };
        deck.shuffle();
        deck
    }
}

impl Deck for FrenchDeck {
    type Item = Card;

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn draw(&mut self) -> Option<Self::Item> {
        self.cards.pop()
    }
}

impl Iterator for FrenchDeck {
    type Item = Card;
    fn next(&mut self) -> Option<Self::Item> {
        self.draw()
    }
}
