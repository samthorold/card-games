use card_games::french::FrenchDeck;

fn main() {
    let deck = FrenchDeck::new();
    for card in deck {
        println!("{:?}", card);
    }
}
