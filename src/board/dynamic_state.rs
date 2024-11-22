use crate::{board::{color::PlayerColor, piece_type::PieceType, square::File}, moves::chess_move::ChessMove};

use super::{piece_type::ColoredPieceType, square::Square};

pub trait DynamicState {
    fn empty() -> Self;
    fn add_piece(&mut self, pt: ColoredPieceType, s: Square);
    fn remove_piece(&mut self, pt: ColoredPieceType, s: Square);
    fn make_move(&mut self, m: ChessMove) {                
        //If castle move rook as well
        if m.is_castle() {
            let (rook_start_file, rook_end_file) = if m.is_long_castle() {                
                (File::A, File::D)
            }   
            else {
                (File::H, File::F)
            };
            
            let rank = m.start.rank();
            let rook_start = Square::from_rank_file(rank, rook_start_file);
            let rook_end = Square::from_rank_file(rank, rook_end_file);   

            self.remove_piece(PieceType::Rook.colored(m.move_piece.color()), rook_start);            
            self.add_piece(PieceType::Rook.colored(m.move_piece.color()), rook_end);
        }
        
        if m.is_direct_capture() {
            self.remove_piece(m.captured_piece, m.end);
        }

        self.remove_piece(m.move_piece, m.start);
        
        if m.is_en_passant() {
            let capture_square = match m.move_piece.color() {
                PlayerColor::White => m.start.up(),
                PlayerColor::Black => m.start.down(),
            };
            
            self.remove_piece(m.move_piece.opposite(), capture_square);
        } else if m.is_promotion() {
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
                (File::A, File::D)
            }   
            else {
                (File::H, File::F)
            };
            
            let rank = m.start.rank();
            let rook_start = Square::from_rank_file(rank, rook_start_file);
            let rook_end = Square::from_rank_file(rank, rook_end_file);   

            self.remove_piece(PieceType::Rook.colored(m.move_piece.color()), rook_end);            
            self.add_piece(PieceType::Rook.colored(m.move_piece.color()), rook_start);
        }
        
        if m.is_direct_capture() {
            self.add_piece(m.captured_piece, m.end);
        }

        self.remove_piece(m.move_piece, m.end);
        
        if m.is_en_passant() {
            let capture_square = match m.move_piece.color() {
                PlayerColor::White => m.start.up(),
                PlayerColor::Black => m.start.down(),
            };
            
            self.add_piece(m.move_piece.opposite(), capture_square);
        } else if m.is_promotion() {
            self.remove_piece(m.promotion_piece, m.end);
        }
        else {
            self.add_piece(m.move_piece, m.start);
        }
    }
}