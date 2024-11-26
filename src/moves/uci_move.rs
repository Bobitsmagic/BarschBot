use crate::board::{piece_type::{ColoredPieceType, PieceType}, player_color::PlayerColor, square::Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UciMove {
    pub start: Square,
    pub end: Square,
    pub promotion_piece: PieceType,
}

impl UciMove {
    pub fn from_str(s: &str) -> UciMove {
        let start = Square::from_str(&s[0..2]);
        let end = Square::from_str(&s[2..4]);
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
        let s = format!("{}{}", self.start.to_string(), self.end.to_string());

        if self.promotion_piece != PieceType::None {
            s + &self.promotion_piece.colored(PlayerColor::Black).to_char().to_string()
        } else {
            s
        }
    }
}