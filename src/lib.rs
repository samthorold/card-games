pub mod french {

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

    pub struct Deck {
        available_cards: Vec<Card>,
        used_cards: Vec<Card>,
    }

    impl Deck {
        pub fn new() -> Deck {
            let mut cards = Vec::new();
            for suit in [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade] {
                cards.push(Card::King(suit));
                cards.push(Card::Queen(suit));
                cards.push(Card::Jack(suit));
                for rank in 1..11 {
                    cards.push(Card::Pip(suit, rank))
                }
            }
            Deck {
                available_cards: cards,
                used_cards: Vec::new(),
            }
        }

        pub fn shuffle(&mut self) {
            let mut rng = thread_rng();
            self.available_cards.shuffle(&mut rng);
        }

        pub fn draw(&mut self) -> Option<Card> {
            let maybe_card = self.available_cards.pop();
            match maybe_card {
                Some(maybe_card) => {
                    self.used_cards.push(maybe_card);
                    Some(maybe_card)
                }
                None => None,
            }
        }
    }
}