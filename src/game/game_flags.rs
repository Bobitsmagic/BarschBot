use crate::{board::{bit_board::BitBoard, piece_type::ColoredPieceType, player_color::PlayerColor, square::{self, Square}}, moves::chess_move::ChessMove};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameFlags {
    pub active_color: PlayerColor,
    pub white_queen_side_castle: bool,
    pub white_king_side_castle: bool,
    pub black_queen_side_castle: bool,
    pub black_king_side_castle: bool,
    pub en_passant_square: i8,
}

impl GameFlags {
    pub fn start_flags() -> GameFlags {
        GameFlags {
            active_color: PlayerColor::White,
            white_queen_side_castle: true,
            white_king_side_castle: true,
            black_queen_side_castle: true,
            black_king_side_castle: true,
            en_passant_square: square::NONE,
        }
    }

    pub fn empty_flags() -> GameFlags {
        GameFlags {
            active_color: PlayerColor::White,
            white_queen_side_castle: false,
            white_king_side_castle: false,
            black_queen_side_castle: false,
            black_king_side_castle: false,
            en_passant_square: square::NONE,
        }
    }

    pub fn make_move(&mut self, m: ChessMove, board: &BitBoard) {
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
        
        match m.start {
            square::A1 => self.white_queen_side_castle = false,
            square::H1 => self.white_king_side_castle = false,
            square::A8 => self.black_queen_side_castle = false,
            square::H8 => self.black_king_side_castle = false,
            _ => (),
        }

        match m.end {
            square::A1 => self.white_queen_side_castle = false,
            square::H1 => self.white_king_side_castle = false,
            square::A8 => self.black_queen_side_castle = false,
            square::H8 => self.black_king_side_castle = false,
            _ => (),
        }
        

        if m.move_piece.is_pawn() && m.start.rank().abs_diff(m.end.rank()) == 2 
            && board.pawn_has_neighbour(m.move_piece.color(), m.end) {

            self.en_passant_square = match m.move_piece.color() {
                PlayerColor::White => m.end.down(),
                PlayerColor::Black => m.end.up(),
            };

        } else {
            self.en_passant_square = square::NONE;
        }
    }
    
    pub fn print(&self) {
        println!("Active color: {:?}, White queen side castle: {}, White king side castle: {}, Black queen side castle: {}, Black king side castle: {}, En passant square: {:?}", 
            self.active_color, self.white_queen_side_castle, self.white_king_side_castle, self.black_queen_side_castle, self.black_king_side_castle, self.en_passant_square)
    }
}