use crate::Card;

#[derive(Debug)]
pub struct Discard {
    total_cards: u8,
    cards: Vec<Card>,
}

impl Discard {
    pub fn new() -> Self {
        Discard {
            total_cards: 0,
            cards: vec![],
        }
    }

    pub fn get_top_card(&self) {
        println!("Top card");
    }
}
