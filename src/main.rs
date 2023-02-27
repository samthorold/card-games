use card_games::french::Deck;

fn main() {
    let deck = Deck::new(true);
    for card in deck {
        println!("{:?}", card);
    }
}
