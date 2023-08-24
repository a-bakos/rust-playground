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
use crate::gui::GUI;
use crate::player::Player;

const TOTAL_CARDS: u8 = 112;
const START_CARDS: u8 = 8;

fn main() {
    GUI::gui_msg(GUI::GameTitle);

    // Define the players
    let player1 = Player::new("Frank".to_string());
    let player2 = Player::new("Grace".to_string());

    // Group them in a vector
    let mut players: Vec<Player> = Vec::new();
    players.push(player1);
    players.push(player2);

    // Initiate gameplay, pass in the players
    let mut gameplay = Game::new(players);

    GUI::gui_msg(GUI::PlayersJoined);

    for player in &gameplay.players {
        println!("\t- {}", player.name);
    }

    GUI::gui_msg(GUI::DeckCardAvailable(gameplay.deck.total_cards));
    gameplay.deal_cards();

    for player in &gameplay.players {
        println!(
            "\n{} has {} card(s):",
            player.name,
            player.number_of_cards()
        );

        helpers::list_cards(&player);
    }

    GUI::gui_msg(GUI::DeckCardAvailable(gameplay.deck.total_cards));
    GUI::gui_msg(GUI::DiscardTopCardIs);
    gameplay
        .discard
        .show_top_card(gameplay.discard.get_top_card());

    gameplay.players[0].discard(0, &mut gameplay.discard);
    GUI::gui_msg(GUI::DiscardTopCardIs);
    gameplay
        .discard
        .show_top_card(gameplay.discard.get_top_card());

    println!("How many cards:");

    helpers::list_cards(&gameplay.players[0]);
    helpers::list_cards(&gameplay.players[1]);

    let player1_cards: Vec<Card> = Game::empty_cards(&mut gameplay.players[0].cards);
    let player2_cards: Vec<Card> = Game::empty_cards(&mut gameplay.players[1].cards);

    Game::swap_cards(player1_cards, &mut gameplay.players[1]);
    Game::swap_cards(player2_cards, &mut gameplay.players[0]);

    GUI::gui_msg(GUI::CardsSwappedBetween(
        gameplay.players[1].name.to_string(),
        gameplay.players[0].name.to_string(),
    ));

    helpers::list_cards(&gameplay.players[0]);
    helpers::list_cards(&gameplay.players[1]);
}
