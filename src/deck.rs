pub trait Deck {
    type Item;
    fn shuffle(&mut self);
    fn draw(&mut self) -> Option<Self::Item>;
}