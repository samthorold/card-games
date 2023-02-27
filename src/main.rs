use card_games::deck::Deck;
use card_games::french::FrenchDeck;

fn draw_all(deck: &mut dyn Deck) {
    loop {
        match deck.draw() {
            Some(card) => println!("{:?}", card),
            None => break,
        }
    }
}

fn main() {
    let mut deck = FrenchDeck::new();
    // for card in deck {
    //     println!("{:?}", card);
    // }
    draw_all(&mut deck);
    draw_all(&mut deck);
}
