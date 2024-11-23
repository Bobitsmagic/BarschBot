use std::ops::{Index, IndexMut};

use colored::{Colorize, CustomColor};

use crate::board::player_color::PlayerColor;

use super::{dynamic_state::DynamicState, piece_type::ColoredPieceType, square::Square};

#[derive(Clone)]
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
    
    pub fn print(&self) {
        self.print_perspective(PlayerColor::White);
    }

    pub fn print_perspective(&self, perspective: PlayerColor) {
        let mut s = String::new();

        for rank in (0..8).rev() {
            for file in 0..8 {

                let square = match perspective {
                    PlayerColor::White => Square::from_rank_file_index(rank, file),
                    PlayerColor::Black => Square::from_rank_file_index(7 - rank, 7 - file),
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
}

impl DynamicState for PieceBoard {
    fn empty() -> Self {
        PieceBoard::empty()
    }

    fn add_piece(&mut self, pt: ColoredPieceType, s: Square) {
        self[s] = pt;
    }

    fn remove_piece(&mut self, _: ColoredPieceType, s: Square) {
        self[s] = ColoredPieceType::None;
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
