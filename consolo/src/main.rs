#[derive(Debug)]
enum Color {
    Yellow,
    Red,
    Blue,
    Green,
    Black,
}

// struct lives on the stack, except for the color property (heao)
#[derive(Debug)]
struct Card {
    color: Color,
    value: u8,
    action: bool,
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
        println!("[ NEW GAME ] No one has joined yet.");
        Players { player_ids: vec![] }
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    id: i8,
    cards: i8, //PlayerCards,
}

impl Player {
    fn new(name: String) -> Player {
        Player {
            name: name,
            id: 1,
            cards: 0,
        }
    }

    fn get(id: i8) {
        println!("Getting player {}", id);
    }

    fn join_game(&self, game: &mut Players) {
        &game.player_ids.push(self.id);
        println!("{} has joined the game.", self.name);
    }

    fn add_card(&mut self, card: Card) {}
}

fn main() {
    let mut all_players = Players::new();

    let player_1 = Player::new(String::from("Abakos"));
    let player_2 = Player::new(String::from("Frank"));
    player_1.join_game(&mut all_players);
    player_2.join_game(&mut all_players);
    println!("The players are: {:?}", all_players);

    let single_card = Card {
        color: Color::Yellow,
        value: 8,
        action: false,
    };
    println!("Hello card: {:?}", single_card);
}
