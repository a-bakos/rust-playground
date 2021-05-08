use crate::Card;

#[derive(Debug)]
pub struct Discard {
    total_cards: u8,
    cards: Vec<Card>,
}
impl Discard {
    pub fn get_top_card(&self) {
        println!("Top card");
    }
}
