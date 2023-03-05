//! Deck traits - the underlying interface for a deck in a card game
use core::fmt::Debug;

pub trait Card: Debug {
    fn string(&self) -> String;
}

pub trait Deck {
    fn shuffle(&mut self);
    fn draw(&mut self) -> Option<Box<dyn Card>>;
}
