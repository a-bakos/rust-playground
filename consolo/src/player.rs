use crate::Card;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub cards: Vec<Card>,
}
impl Player {
    pub fn number_of_cards(&self) -> usize {
        self.cards.len()
    }

    pub fn new(name: String) -> Self {
        Self {
            name,
            cards: vec![],
        }
    }

    pub fn discard(&mut self) {
        self.cards.pop();
    }
}
