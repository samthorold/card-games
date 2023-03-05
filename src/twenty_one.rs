//! 21 card game implementation - a simpler version of blackjack

use crate::deck::{Card as CardTrait, Deck as DeckTrait};
use crate::french::FrenchDeck as Deck;

struct Hand<'a> {
    cards: Vec<&'a Box<dyn CardTrait>>,
}

impl<'a> Hand<'a> {
    fn new() -> Hand<'a> {
        Hand { cards: Vec::new() }
    }

    fn add(&'a mut self, card: &'a Box<dyn CardTrait>) {
        self.cards.push(card);
    }
}
struct Player<'a> {
    hand: Hand<'a>,
}

impl<'a> Player<'a> {
    fn new() -> Player<'a> {
        Player { hand: Hand::new() }
    }

    fn add_to_hand(&'a mut self, card: &'a Box<dyn CardTrait>) {
        self.hand.add(card);
    }
}

pub struct Game<'a> {
    deck: Deck,
    players: Vec<Player<'a>>,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'static> {
        Game {
            deck: Deck::new(),
            players: vec![Player::new(), Player::new()],
        }
    }

    pub fn turn(&'a mut self) {
        let mut i = 0;
        for player in &mut self.players {
            i += 1;
            loop {
                match self.deck.draw() {
                    Some(card) => {
                        println!("player {} drew {:?}", i, card);
                        player.add_to_hand(&card);
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
