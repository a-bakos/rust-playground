pub enum GUI {
    GameTitle,
    CardsSwappedBetween(String, String),
    DeckCardAvailable(u8),
    DiscardEmpty,
    DiscardTopCardIs,
    PlayersJoined,
}

impl GUI {
    pub fn gui_msg(gui_msg: GUI) {
        match gui_msg {
            GUI::GameTitle => println!("\n..::| C O N S O L O |::..\n"),
            GUI::PlayersJoined => println!("The following players have joined the game:"),
            GUI::DeckCardAvailable(cards) => println!("\nThe deck has {} cards", cards),
            GUI::DiscardEmpty => println!("The discard pile is empty"),
            GUI::DiscardTopCardIs => println!("the top card in the discard pile is:"),
            GUI::CardsSwappedBetween(pl1, pl2) => {
                println!("{} and {} have swapped cards", pl1, pl2)
            }
        }
    }
}
