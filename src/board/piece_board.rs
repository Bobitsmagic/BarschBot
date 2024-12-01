use std::ops::{Index, IndexMut};

use colored::{Colorize, CustomColor};

use crate::{board::player_color::PlayerColor, moves::chess_move::ChessMove};

use super::{bit_board::BitBoard, dynamic_state::DynamicState, piece_type::ColoredPieceType, square::{self, VALID_SQUARES}};
use crate::board::square::Square;

#[derive(Clone, PartialEq, Eq)]
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

        pb[square::A1] = ColoredPieceType::WhiteRook;
        pb[square::B1] = ColoredPieceType::WhiteKnight;
        pb[square::C1] = ColoredPieceType::WhiteBishop;
        pb[square::D1] = ColoredPieceType::WhiteQueen;
        pb[square::E1] = ColoredPieceType::WhiteKing;
        pb[square::F1] = ColoredPieceType::WhiteBishop;
        pb[square::G1] = ColoredPieceType::WhiteKnight;
        pb[square::H1] = ColoredPieceType::WhiteRook;

        pb[square::A8] = ColoredPieceType::BlackRook;
        pb[square::B8] = ColoredPieceType::BlackKnight;
        pb[square::C8] = ColoredPieceType::BlackBishop;
        pb[square::D8] = ColoredPieceType::BlackQueen;
        pb[square::E8] = ColoredPieceType::BlackKing;
        pb[square::F8] = ColoredPieceType::BlackBishop;
        pb[square::G8] = ColoredPieceType::BlackKnight;
        pb[square::H8] = ColoredPieceType::BlackRook;

        for s in square::A2..=square::H2 {
            pb[s] = ColoredPieceType::WhitePawn;
        }

        for s in square::A7..=square::H7 {
            pb[s] = ColoredPieceType::BlackPawn;
        }
        
        return pb;
    }
    
    pub fn print(&self) {
        self.print_perspective(PlayerColor::White);
    }

    pub fn from_bit_board(bit_board: &BitBoard) -> PieceBoard {
        let mut board = PieceBoard::empty();
        for s in VALID_SQUARES {
            board[s] = bit_board.get_colored_piecetype(s);
        }

        return board;
    }

    pub fn print_perspective(&self, perspective: PlayerColor) {
        let mut s = String::new();

        for rank in (0..8).rev() {
            for file in 0..8 {

                let square = match perspective {
                    PlayerColor::White => square::from_file_rank(file, rank),
                    PlayerColor::Black => square::from_file_rank(7 - file, 7 - rank),
                };

                let piece = self[square];

                let square_color = if square.is_light() { CustomColor::new(0, 0, 0)} else { CustomColor::new(25, 25, 25,)};
                
                if piece.is_none() {
                    s += &format!("{}", "  ".on_custom_color(square_color));
                }
                else {
                    let piece_color = match piece.color() {
                        PlayerColor::White => CustomColor::new(220, 220, 220),
                        PlayerColor::Black => CustomColor::new(200, 200, 200),
                    };
    
    
                    s += &format!("{}", format!("{} ", piece.to_symbol()).custom_color(piece_color).on_custom_color(square_color));
                }

            }
            s += "\n";
        }

        println!("{}", s);
    }
    
    pub fn get_move(&self, start: i8, target: i8) -> ChessMove {
        ChessMove::new(start, target, self[start], self[target])
    }
}

impl DynamicState for PieceBoard {
    fn empty() -> Self {
        PieceBoard::empty()
    }

    fn add_piece(&mut self, pt: ColoredPieceType, s: i8) {
        self[s] = pt;
    }

    fn remove_piece(&mut self, _: ColoredPieceType, s: i8) {
        self[s] = ColoredPieceType::None;
    }
}

impl Index<i8> for PieceBoard {
    type Output = ColoredPieceType;
    
    fn index(&self, square: i8) -> &Self::Output {
        &self.squares[square as usize]
    }
}

impl IndexMut<i8> for PieceBoard {
    fn index_mut(&mut self, square: i8) -> &mut Self::Output {
        &mut self.squares[square as usize]
    }
}
