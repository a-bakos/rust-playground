use crate::discard::Discard;
use crate::Card;
use crate::Deck;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub cards: Vec<Card>,
    pub points: u8,
}
impl Player {
    pub fn number_of_cards(&self) -> usize {
        self.cards.len()
    }

    pub fn new(name: String) -> Self {
        Self {
            name,
            cards: vec![],
            points: 0,
        }
    }

    /// Remove/Discard a card from the player's hand by card position
    pub fn discard(&mut self, card_position: usize, discard_pile: &mut Discard) {
        let to_discard: Card = self.cards.remove(card_position);

        println!("{:?}", to_discard);

        discard_pile.cards.push(to_discard);
        discard_pile.total_cards += 1;
    }

    pub fn take_two(&self, deck: &mut Deck) {
        let mut counter = 0;
        loop {
            if counter == 2 {
                break;
            }

            let card = match deck.the_cards.pop() {
                Some(the_card) => the_card,
                None => println!("reset discard"),
            };
            self.cards.push(the_card);
        }
    }
}
