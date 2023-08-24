use crate::Card;
use crate::Color;
use crate::Player;

pub fn show_card_face(card: &Card) {
    let color = match card.color {
        Color::Red => "R",
        Color::Yellow => "Y",
        Color::Blue => "B",
        Color::Green => "G",
        Color::Black => "A",
    };
    print!("[{}{}] ", color, card.value);
}

pub fn list_cards(player: &Player) {
    let mut card_counter: u8 = 0;
    for card in player.cards.iter() {
        card_counter += 1;
        print!("{}. ", card_counter);
        show_card_face(&card);
    }
    println!("");
}
