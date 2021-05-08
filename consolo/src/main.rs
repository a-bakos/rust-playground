mod card;
mod deck;
mod discard;
mod game;
mod player;

use crate::card::{Card, Color};
use crate::deck::Deck;
use crate::game::Game;
use crate::player::Player;

const TOTAL_CARDS: u8 = 112;

fn main() {
    println!("\n..::| C O N S O L O |::..\n");

    // Define the players
    let player1 = Player::new("Frank".to_string());
    let player2 = Player::new("Grace".to_string());

    // Group them in a vector
    let mut players: Vec<Player> = Vec::new();
    players.push(player1);
    players.push(player2);

    // Initiate gameplay, pass in the players
    let mut gameplay = Game::new(players);
    println!("The following players have joined the game:");
    for player in &gameplay.players {
        println!("\t- {}", player.name);
    }

    println!("\nThe deck has {} cards", gameplay.deck.total_cards);
    gameplay.deal_cards();

    for player in &gameplay.players {
        println!(
            "\n{} has {} card(s):",
            player.name,
            player.number_of_cards()
        );

        for card in player.cards.iter() {
            let color = match card.color {
                Color::Red => "R",
                Color::Yellow => "Y",
                Color::Blue => "B",
                Color::Green => "G",
                Color::Black => "A",
            };
            print!("[{}{}] ", color, card.value);
        }
        println!("");
    }

    println!("\nDeck has {:?} cards", gameplay.deck.total_cards);
}
