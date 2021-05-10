#[derive(Copy, Clone, Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Black,
}

#[derive(Copy, Clone, Debug)]
pub struct Card {
    pub value: u8,
    pub color: Color,
}

impl Card {
    pub fn get_value(&self) -> u8 {
        self.value
    }

    pub fn get_color(&self) -> String {
        match self.color {
            Color::Red => "Red".to_string(),
            Color::Green => "Green".to_string(),
            Color::Blue => "Blue".to_string(),
            Color::Yellow => "Yellow".to_string(),
            Color::Black => "Black".to_string(),
        }
    }
}
