mod card;
mod deck;
mod discard;
mod game;
mod gui;
mod helpers;
mod player;

use crate::card::{Card, Color};
use crate::deck::Deck;
use crate::game::Game;
use crate::player::Player;

const TOTAL_CARDS: u8 = 112;
const START_CARDS: u8 = 8;

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

        helpers::list_cards(&player);
    }

    println!("\nDeck has {:?} cards", gameplay.deck.total_cards);

    println!("the top card on the discard pile is:");
    gameplay
        .discard
        .show_top_card(gameplay.discard.get_top_card());

    gameplay.players[0].discard(0, &mut gameplay.discard);
    println!(
        "\n{} has {} card(s):",
        gameplay.players[0].name,
        gameplay.players[0].number_of_cards()
    );
    //helpers::list_cards(&gameplay.players[0]);

    println!("the top card on the discard pile is:");
    gameplay
        .discard
        .show_top_card(gameplay.discard.get_top_card());
}
