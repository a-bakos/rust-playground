use crate::TOTAL_CARDS;
use crate::{Card, Color};

#[derive(Debug)]
pub struct Deck {
    pub total_cards: u8,
    pub the_cards: Vec<Card>,
}
impl Deck {
    pub fn new() -> Self {
        Self {
            total_cards: TOTAL_CARDS,
            the_cards: vec![],
        }
    }

    pub fn deal() -> Vec<Card> {
        vec![Card {
            value: 10,
            color: Color::Red,
        }]
    }

    fn shuffle() {}
}
