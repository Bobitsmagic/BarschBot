use crate::board::{piece_type::ColoredPieceType, square::Square};



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChessMove {
    pub start: Square,
    pub end: Square,
    pub move_piece: ColoredPieceType,
    pub captured_piece: ColoredPieceType,
    pub promotion_piece: ColoredPieceType,
}

impl ChessMove {
    pub fn new(start: Square, end: Square, move_piece: ColoredPieceType, captured_piece: ColoredPieceType) -> ChessMove {
        ChessMove {
            start,
            end,
            move_piece,
            captured_piece,
            promotion_piece: ColoredPieceType::None,
        }
    }

    pub fn new_pawn(start: Square, end: Square, move_piece: ColoredPieceType, captured_piece: ColoredPieceType, promotion_piece: ColoredPieceType) -> ChessMove {
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

    pub fn print(&self) {
        println!("{}{}{}", self.move_piece.to_char(), self.start.to_string(), self.end.to_string());
    }
}