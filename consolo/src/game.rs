use crate::player::Player;
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
}
