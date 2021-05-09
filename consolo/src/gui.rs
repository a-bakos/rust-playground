pub enum GUI {
    DiscardEmpty,
}

impl GUI {
    pub fn gui_msg(gui_msg: GUI) {
        match gui_msg {
            GUI::DiscardEmpty => println!("The discard pile is empty"),
        }
    }
}
