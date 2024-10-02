use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    White,
    Black,
}

pub const ALL_COLORS: [Color; 2] = [Color::White, Color::Black];

impl Color {
    pub fn new(is_white: bool) -> Color {
        if is_white {
            Color::White
        }
        else {
            Color::Black
        }
    }
    pub fn is_white(&self) -> bool {
        *self == Color::White
    }
    pub fn is_black(&self) -> bool {
        *self == Color::Black
    }
}

impl ops::Not for Color {
    type Output = Color;

    fn not(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

