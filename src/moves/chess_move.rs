use crate::board::{piece_type::{ColoredPieceType, PieceType}, square::Square};

use super::uci_move::UciMove;


pub const NULL_MOVE: ChessMove = ChessMove {
    start: Square::A1,
    end: Square::A1,
    move_piece: ColoredPieceType::None,
    captured_piece: ColoredPieceType::None,
    promotion_piece: ColoredPieceType::None,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChessMove {
    pub start: Square,
    pub end: Square,
    pub move_piece: ColoredPieceType,
    pub captured_piece: ColoredPieceType,
    pub promotion_piece: ColoredPieceType,
}

impl Default for ChessMove {
    fn default() -> Self {
        ChessMove {
            start: Square::A1,
            end: Square::A1,
            move_piece: ColoredPieceType::None,
            captured_piece: ColoredPieceType::None,
            promotion_piece: ColoredPieceType::None,
        }
    }
}

impl ChessMove {
    pub fn new(start: Square, end: Square, move_piece: ColoredPieceType, captured_piece: ColoredPieceType) -> ChessMove {
        debug_assert!(move_piece != ColoredPieceType::None);
        debug_assert!(captured_piece.piece_type() != PieceType::King);

        ChessMove {
            start,
            end,
            move_piece,
            captured_piece,
            promotion_piece: ColoredPieceType::None,
        }
    }

    pub fn new_pawn(start: Square, end: Square, move_piece: ColoredPieceType, captured_piece: ColoredPieceType, promotion_piece: ColoredPieceType) -> ChessMove {
        debug_assert!(captured_piece.piece_type() != PieceType::King);

        ChessMove {
            start,
            end,
            move_piece,
            captured_piece,
            promotion_piece,
        }
    }

    pub fn is_direct_capture(&self) -> bool {
        self.captured_piece != ColoredPieceType::None
    }

    pub fn is_en_passant(&self) -> bool {
        self.move_piece.is_pawn() && self.captured_piece.is_none() && self.end.file_index() != self.start.file_index()
    }

    pub fn is_capture(&self) -> bool {
        self.is_direct_capture() || self.is_en_passant()
    }

    pub fn is_promotion(&self) -> bool {
        self.promotion_piece != ColoredPieceType::None
    }

    pub fn is_long_castle(&self) -> bool {
        self.move_piece.is_king() && self.start.file_index() == 2 + self.end.file_index()
    }

    pub fn is_short_castle(&self) -> bool {
        self.move_piece.is_king() && self.start.file_index() + 2 == self.end.file_index()
    }

    pub fn is_castle(&self) -> bool {
        self.move_piece.is_king() && self.start.file_index().abs_diff(self.end.file_index()) == 2
    }

    pub fn uci_move(&self) -> UciMove {
        UciMove {
            start: self.start,
            end: self.end,
            promotion_piece: self.promotion_piece.piece_type(),
        }
    }

    pub fn print(&self) {
        let mut s = String::new();
        s += &self.move_piece.to_char().to_string();
        s += &self.start.to_string();

        if self.is_direct_capture() {
            s += "x";
        } else {
            s += "-";
        }

        if self.is_direct_capture() {
            s += &self.captured_piece.to_char().to_string();
        }

        s += &self.end.to_string(); 
        
        println!("{}", s);
    }
}