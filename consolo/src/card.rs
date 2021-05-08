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
