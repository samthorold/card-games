//! 21 card game implementation - a simpler version of blackjack

use crate::deck::{Card as CardTrait, Deck as DeckTrait};
use crate::french::FrenchDeck as Deck;

struct Hand {
    cards: Vec<Box<dyn CardTrait>>,
}

impl Hand {
    fn new() -> Hand {
        Hand { cards: Vec::new() }
    }

    fn add(&mut self, card: Box<dyn CardTrait>) {
        self.cards.push(card);
    }

    fn len(&self) -> usize {
        self.cards.len()
    }
}
struct Player {
    hand: Hand,
}

impl Player {
    fn new() -> Player {
        Player { hand: Hand::new() }
    }

    fn add_to_hand(&mut self, card: Box<dyn CardTrait>) {
        self.hand.add(card);
    }
}

pub struct Game {
    deck: Deck,
    players: Vec<Player>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            deck: Deck::new(),
            players: vec![Player::new(), Player::new()],
        }
    }

    pub fn turn(&mut self) {
        let mut i = 0;
        for player in &mut self.players {
            i += 1;
            loop {
                match self.deck.draw() {
                    Some(card) => {
                        println!("player {} drew {:?}", i, card);
                        match player.hand.len() {
                            0..=3 => player.add_to_hand(card),
                            _ => break,
                        }
                    }
                    _ => {
                        println!("no more cards");
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_game() {
        Game::new();
    }
}
