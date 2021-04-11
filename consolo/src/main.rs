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
impl Card {
    fn get_color(&self) {
        let color = match self.color {
            Color::Yellow => "Yellow",
            Color::Red => "Red",
            Color::Blue => "Blue",
            Color::Green => "Green",
            Color::Black => "Black",
        };

        println!("Color is {}", color);
    }
}

fn main() {
    let single_card = Card {
        color: Color::Yellow,
        value: 8,
        action: false,
    };
    println!("Hello card: {:?}", single_card);
    single_card.get_color();
}
