pub enum GUI {
    GameTitle,
    DiscardEmpty,
    DiscardTopCardIs,
    PlayersJoined,
}

impl GUI {
    pub fn gui_msg(gui_msg: GUI) {
        match gui_msg {
            GUI::DiscardEmpty => println!("The discard pile is empty"),
            GUI::DiscardTopCardIs => println!("the top card on the discard pile is:"),
            GUI::PlayersJoined => println!("The following players have joined the game:"),
            GUI::GameTitle => println!("\n..::| C O N S O L O |::..\n"),
        }
    }
}
