const TOTAL_CARDS: u8 = 112;
#[derive(Debug)]
struct Counter {
    steps: u8,
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Black,
}

#[derive(Debug)]
struct Card {
    value: u8,
    color: Color,
}

#[derive(Debug)]
struct Player {
    name: String,
    cards: Vec<Card>,
}
impl Player {
    fn number_of_cards(&self) -> usize {
        self.cards.len()
    }

    fn new(name: String) -> Self {
        Self {
            name,
            cards: Deck::deal(),
        }
    }
}

#[derive(Debug)]
struct Deck {
    total_cards: u8,
}
impl Deck {
    fn deal() -> Vec<Card> {
        vec![Card {
            value: 10,
            color: Color::Red,
        }]
    }
}

fn main() {
    let mut deck = Deck {
        total_cards: TOTAL_CARDS,
    };

    let player1 = Player {
        name: "Frank".to_string(),
        cards: vec![
            Card {
                value: 5,
                color: Color::Red,
            },
            Card {
                value: 2,
                color: Color::Green,
            },
            Card {
                value: 3,
                color: Color::Yellow,
            },
        ],
    };

    let play1_cards = player1.number_of_cards();
    println!("{} has {} cards:", player1.name, play1_cards);
    for card in player1.cards.iter() {
        let color = match card.color {
            Color::Red => "R",
            Color::Yellow => "Y",
            Color::Blue => "B",
            Color::Green => "G",
            Color::Black => "A",
        };
        print!("[{}{}] ", color, card.value);
    }

    deck.total_cards -= player1.number_of_cards() as u8;

    println!("\n{:?}", deck.total_cards);

    let player2 = Player::new("Thomas".to_string());
    println!("{:?}", player2.number_of_cards());
}
