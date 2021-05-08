use crate::player::Player;
use crate::Card;
use crate::Color;
use crate::Deck;

#[derive(Debug)]
pub struct Game {
    pub start: String,
    pub steps: u8,
    pub players: Vec<Player>,
    pub deck: Deck,
}

impl Game {
    pub fn new(players: Vec<Player>) -> Self {
        Self {
            start: "timestamp".to_string(), // TODO
            steps: 0,
            players,
            deck: Deck::new(),
        }
    }

    pub fn deal_cards(&mut self) {
        for mut player in self.players.iter_mut() {
            player.cards = vec![Card {
                value: 10,
                color: Color::Red,
            }];

            self.deck.total_cards -= 1;
        }
    }
}
