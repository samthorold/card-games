use card_games::french::Deck;

fn main() {

    let mut deck = Deck::new();
    deck.shuffle();
    for _ in 0..54 {
        match deck.draw() {
            Some(card) => println!("{:?}", card),
            None => println!("No more cards"),
        }
    }
}
