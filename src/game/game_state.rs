use crate::board::{color::PlayerColor, square::Square};

pub struct GameState {
    pub active_color: PlayerColor,
    pub white_queen_side_castle: bool,
    pub white_king_side_castle: bool,
    pub black_queen_side_castle: bool,
    pub black_king_side_castle: bool,
    pub en_passant_square: Square,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            active_color: PlayerColor::White,
            white_queen_side_castle: true,
            white_king_side_castle: true,
            black_queen_side_castle: true,
            black_king_side_castle: true,
            en_passant_square: Square::None,
        }
    }
}