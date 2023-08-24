use crate::gui::GUI;
use crate::helpers::show_card_face;
use crate::Card;

#[derive(Debug)]
pub struct Discard {
    pub total_cards: u8,
    pub cards: Vec<Card>,
}

impl Discard {
    pub fn new() -> Self {
        Discard {
            total_cards: 0,
            cards: vec![],
        }
    }

    /// Get the top card struct option from the discard pile
    pub fn get_top_card(&self) -> Option<&Card> {
        self.cards.last()
    }

    /// Show top card value if there's a card
    pub fn show_top_card(&self, top_card: Option<&Card>) {
        match top_card {
            Some(card) => {
                show_card_face(&card);
            }
            None => GUI::gui_msg(GUI::DiscardEmpty),
        }
    }
}
