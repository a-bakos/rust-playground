#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Black,
}

#[derive(Debug)]
pub struct Card {
    pub value: u8,
    pub color: Color,
}
