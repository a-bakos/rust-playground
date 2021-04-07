#[derive(Debug)]
struct Card {
    // struct lives on the stack, except for the color property (heao)
    color: String,
    value: u8,
    action: bool,
}

#[derive(Debug)]
struct CardPool {
    //	cards:
}

#[derive(Debug)]
struct PlayerCards {
    player_id: i8,
    cards: Vec<Card>,
}

#[derive(Debug)]
struct Players {
    player_ids: Vec<i8>,
}
impl Players {
    fn new() -> Players {
        Players {
            player_ids: vec![0],
        }
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    id: i8,
    cards: PlayerCards,
}

impl Player {
    fn new(name: String, cards: PlayerCards) -> Player {
        Player {
            name: name,
            id: 1,
            cards: cards,
        }
    }

    fn add_card(&mut self, card: Card) {}
}

fn main() {
    let game_players = Players::new();
    println!("The players: {:?}", game_players);

    //let player_1 = Player::new(String::from("Abakos"));
    //println!("Hello player: {:?}", player_1);

    let single_card = Card {
        color: String::from("yellow"),
        value: 8,
        action: false,
    };
    println!("Hello card: {:?}", single_card);
}
