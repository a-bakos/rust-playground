use crate::discard::Discard;
use crate::player::Player;
use crate::Card;
use crate::Color;
use crate::Deck;
use crate::START_CARDS;
use rand::prelude::*;

#[derive(Debug)]
pub struct Game {
    pub start: String,
    pub steps: u8,
    pub players: Vec<Player>,
    pub deck: Deck,
    pub discard: Discard,
}

impl Game {
    pub fn new(players: Vec<Player>) -> Self {
        Self {
            start: "timestamp".to_string(), // TODO
            steps: 0,
            players,
            deck: Deck::new(),
            discard: Discard::new(),
        }
    }

    pub fn deal_cards(&mut self) {
        for mut player in self.players.iter_mut() {
            let mut i = 0;
            while i <= START_CARDS - 1 {
                let card = Card {
                    value: rand::thread_rng().gen_range(1..10),
                    color: Color::Red,
                };
                player.cards.push(card);
                self.deck.total_cards -= 1;
                i += 1;
            }
        }
    }
}
