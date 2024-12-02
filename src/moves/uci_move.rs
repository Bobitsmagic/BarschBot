use crate::board::{piece_type::{ColoredPieceType, PieceType}, player_color::PlayerColor, square::{self, Square}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UciMove {
    pub start: i8,
    pub end: i8,
    pub promotion_piece: PieceType,
}

impl UciMove {
    pub fn new(start: i8, end: i8, promotion_piece: PieceType) -> UciMove {
        UciMove {
            start,
            end,
            promotion_piece,
        }
    }

    pub fn from_str(s: &str) -> UciMove {
        let start = square::from_str(&s[0..2]);
        let end = square::from_str(&s[2..4]);
        let promotion_piece = match s.len() {
            5 => ColoredPieceType::from_char(s.chars().nth(4).unwrap()),
            _ => ColoredPieceType::None,
        }.piece_type();

        UciMove {
            start,
            end,
            promotion_piece,
        }
    }
}

impl ToString for UciMove {
    fn to_string(&self) -> String {
        let s = format!("{}{}", self.start.square_string(), self.end.square_string());

        if self.promotion_piece != PieceType::None {
            s + &self.promotion_piece.colored(PlayerColor::Black).to_char().to_string()
        } else {
            s
        }
    }
}