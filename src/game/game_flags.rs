use crate::{board::{player_color::PlayerColor, piece_type::ColoredPieceType, square::Square}, moves::chess_move::ChessMove};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GameFlags {
    pub active_color: PlayerColor,
    pub white_queen_side_castle: bool,
    pub white_king_side_castle: bool,
    pub black_queen_side_castle: bool,
    pub black_king_side_castle: bool,
    pub en_passant_square: Square,
}

impl GameFlags {
    pub fn start_flags() -> GameFlags {
        GameFlags {
            active_color: PlayerColor::White,
            white_queen_side_castle: true,
            white_king_side_castle: true,
            black_queen_side_castle: true,
            black_king_side_castle: true,
            en_passant_square: Square::None,
        }
    }

    pub fn make_move(&mut self, m: ChessMove) {
        self.active_color = !self.active_color;

        match m.move_piece {
            ColoredPieceType::WhiteKing => {
                self.white_king_side_castle = false;
                self.white_queen_side_castle = false;
            }
            ColoredPieceType::BlackKing => {
                self.black_king_side_castle = false;
                self.black_queen_side_castle = false;
            }
            _ => (),
        } 
        
        if m.move_piece.is_rook() {
            match m.start {
                Square::A1 => self.white_queen_side_castle = false,
                Square::H1 => self.white_king_side_castle = false,
                Square::A8 => self.black_queen_side_castle = false,
                Square::H8 => self.black_king_side_castle = false,
                _ => (),
            }
        }

        if m.move_piece.is_pawn() && m.start.rank_index().abs_diff(m.end.rank_index()) == 2 {
            self.en_passant_square = match m.move_piece.color() {
                PlayerColor::White => m.start.down(),
                PlayerColor::Black => m.start.up(),
            };
        } else {
            self.en_passant_square = Square::None;
        }
    }
}