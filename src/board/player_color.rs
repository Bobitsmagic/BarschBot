use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayerColor {
    White,
    Black,
}

pub const ALL_COLORS: [PlayerColor; 2] = [PlayerColor::White, PlayerColor::Black];

impl PlayerColor {
    pub fn new(is_white: bool) -> PlayerColor {
        if is_white {
            PlayerColor::White
        }
        else {
            PlayerColor::Black
        }
    }
    pub fn is_white(&self) -> bool {
        *self == PlayerColor::White
    }
    pub fn is_black(&self) -> bool {
        *self == PlayerColor::Black
    }
}

impl ops::Not for PlayerColor {
    type Output = PlayerColor;

    fn not(self) -> PlayerColor {
        match self {
            PlayerColor::White => PlayerColor::Black,
            PlayerColor::Black => PlayerColor::White,
        }
    }
}

impl ToString for PlayerColor {
    fn to_string(&self) -> String {
        match self {
            PlayerColor::White => "w".to_string(),
            PlayerColor::Black => "b".to_string(),
        }
    }
}

