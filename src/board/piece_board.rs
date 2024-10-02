use std::ops::{Index, IndexMut};

use crate::board::{color::Color, square::File};

use super::{chess_move::ChessMove, piece_type::ColoredPieceType, square::Square};

pub struct PieceBoard {
    squares: [ColoredPieceType; 64],
}

impl PieceBoard {
    pub fn empty() -> PieceBoard {
        PieceBoard {
            squares: [ColoredPieceType::None; 64],
        }
    }

    pub fn start_position() -> PieceBoard {
        let mut pb = PieceBoard::empty();

        pb[Square::A1] = ColoredPieceType::WhiteRook;
        pb[Square::B1] = ColoredPieceType::WhiteKnight;
        pb[Square::C1] = ColoredPieceType::WhiteBishop;
        pb[Square::D1] = ColoredPieceType::WhiteQueen;
        pb[Square::E1] = ColoredPieceType::WhiteKing;
        pb[Square::F1] = ColoredPieceType::WhiteBishop;
        pb[Square::G1] = ColoredPieceType::WhiteKnight;
        pb[Square::H1] = ColoredPieceType::WhiteRook;

        pb[Square::A8] = ColoredPieceType::BlackRook;
        pb[Square::B8] = ColoredPieceType::BlackKnight;
        pb[Square::C8] = ColoredPieceType::BlackBishop;
        pb[Square::D8] = ColoredPieceType::BlackQueen;
        pb[Square::E8] = ColoredPieceType::BlackKing;
        pb[Square::F8] = ColoredPieceType::BlackBishop;
        pb[Square::G8] = ColoredPieceType::BlackKnight;
        pb[Square::H8] = ColoredPieceType::BlackRook;

        for s in Square::A2.rectangle_to(Square::H2) {
            pb[s] = ColoredPieceType::WhitePawn;
        }

        for s in Square::A7.rectangle_to(Square::H7) {
            pb[s] = ColoredPieceType::BlackPawn;
        }
        
        return pb;
    }


    pub fn set_piece(&mut self, pt: ColoredPieceType, s: Square) {
        self[s] = pt;
    }
    pub fn remove_piece(&mut self, s: Square) {
        self[s] = ColoredPieceType::None
    }
    pub fn move_piece(&mut self, start: Square, end: Square) {
        self[end] = self[start];
        self[start] = ColoredPieceType::None;
    }
    
    pub fn make_move(&mut self, m: ChessMove) {
        debug_assert!(m.start != m.end, "Invalid move: start == end");
        debug_assert!(self[m.start] == m.move_piece, "Invalid move: start piece is not the same as move piece");
        
        if m.is_castle() {
            let (rook_start_file, rook_end_file) = if m.is_big_castle() {
                debug_assert!(self[m.start.left()].is_none(), "Invalid move: {} square is not empty", m.start.left().to_string());
                debug_assert!(self[m.start.left().left()].is_none(), "Invalid move: {} square is not empty", m.start.left().left().to_string());
                debug_assert!(self[m.start.left().left().left()].is_none(), "Invalid move: {} square is not empty", m.start.left().left().left().to_string());
                
                (File::A, File::D)
            }   
            else {
                debug_assert!(self[m.start.right()].is_none(), "Invalid move: {} square is not empty", m.start.right().to_string());
                debug_assert!(self[m.start.right().right()].is_none(), "Invalid move: {} square is not empty", m.start.right().right().to_string());

                (File::H, File::F)
            };
            
            let rank = m.start.rank();
            let rook_start = Square::from_rank_file(rank, rook_start_file);
            let rook_end = Square::from_rank_file(rank, rook_end_file);   
            
            self[rook_end] = self[rook_start];
            self[rook_start] = ColoredPieceType::None;
        }
        
        self[m.end] = m.move_piece;
        self[m.start] = ColoredPieceType::None;
        
        if m.is_en_passant() {
            let capture_square = match m.move_piece.color() {
                Color::White => m.start.up(),
                Color::Black => m.start.down(),
            };
            
            debug_assert!(self[capture_square] == m.move_piece.opposite(), "Invalid move: en passant capture square is not a pawn");
            
            self[capture_square] = ColoredPieceType::None;
        } else if m.is_promotion() {
            self[m.end] = m.promotion_piece;
        }        
    }

    pub fn print(&self) {
        self.print_perspective(Color::White);
    }
    pub fn print_perspective(&self, perspective: Color) {
        let mut s = String::new();

        for rank in (0..8).rev() {
            for file in 0..8 {

                let square = match perspective {
                    Color::White => Square::from_rank_file_index(rank, file),
                    Color::Black => Square::from_rank_file_index(7 - rank, 7 - file),
                };

                let piece = self[square];

                if piece == ColoredPieceType::None {
                    s += &format!("{} ", square.to_smybol());
                } else {
                    s += &format!("{} ", piece.to_symbol());
                }
            }
            s += "\n";
        }

        println!("{}", s);
    }
}

impl Index<Square> for PieceBoard {
    type Output = ColoredPieceType;
    
    fn index(&self, square: Square) -> &Self::Output {
        &self.squares[square as usize]
    }
}

impl IndexMut<Square> for PieceBoard {
    fn index_mut(&mut self, square: Square) -> &mut Self::Output {
        &mut self.squares[square as usize]
    }
}
