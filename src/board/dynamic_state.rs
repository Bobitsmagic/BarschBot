use crate::{board::{player_color::PlayerColor, piece_type::PieceType}, moves::chess_move::ChessMove};

use super::{piece_type::ColoredPieceType, square::{self, Square}};

pub trait DynamicState {
    fn empty() -> Self;
    fn add_piece(&mut self, pt: ColoredPieceType, s: i8);
    fn remove_piece(&mut self, pt: ColoredPieceType, s: i8);
    fn make_move(&mut self, m: ChessMove) {                
        //If castle move rook as well
        if m.is_castle() {
            let (rook_start_file, rook_end_file) = if m.is_long_castle() {                
                (0, 3) //A, D
            }   
            else {
                (7, 5) //H, F
            };
            
            let rank = m.start.rank();
            let rook_start = square::from_file_rank(rook_start_file, rank);
            let rook_end = square::from_file_rank(rook_end_file, rank);

            self.remove_piece(PieceType::Rook.colored(m.move_piece.color()), rook_start);            
            self.add_piece(PieceType::Rook.colored(m.move_piece.color()), rook_end);
        }
        
        if m.is_direct_capture() {
            self.remove_piece(m.captured_piece, m.end);
        }

        self.remove_piece(m.move_piece, m.start);
        
        if m.is_en_passant() {
            let capture_i8 = match m.move_piece.color() {
                PlayerColor::White => m.end.down(),
                PlayerColor::Black => m.end.up(),
            };
            
            self.remove_piece(m.move_piece.opposite(), capture_i8);
        }
        
        if m.is_promotion() {
            self.add_piece(m.promotion_piece, m.end);
        }
        else {
            self.add_piece(m.move_piece, m.end);
        }
    }

    //[TODO] unit test for undo_move
    fn undo_move(&mut self, m: ChessMove) {
        if m.is_castle() {
            let (rook_start_file, rook_end_file) = if m.is_long_castle() {                
                (0, 3) //A, D
            }   
            else {
                (7, 5) //H, F
            };
            
            let rank = m.start.rank();
            let rook_start = square::from_file_rank(rook_start_file, rank);
            let rook_end = square::from_file_rank(rook_end_file, rank); 

            self.remove_piece(PieceType::Rook.colored(m.move_piece.color()), rook_end);            
            self.add_piece(PieceType::Rook.colored(m.move_piece.color()), rook_start);
        }
        
        if m.is_promotion() {
            self.remove_piece(m.promotion_piece, m.end);
        }
        else {
            self.remove_piece(m.move_piece, m.end);
        }

        if m.is_direct_capture() {
            self.add_piece(m.captured_piece, m.end);
        }
        
        if m.is_en_passant() {
            let capture_i8 = match m.move_piece.color() {
                PlayerColor::White => m.end.down(),
                PlayerColor::Black => m.end.up(),
            };
            
            self.add_piece(m.move_piece.opposite(), capture_i8);
        }
        
        self.add_piece(m.move_piece, m.start);
    }
}