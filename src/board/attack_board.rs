use std::u64;

use rand::RngCore;

use crate::{board::{bit_array_lookup::{DIAGONAL_MOVES, KING_MOVES, KNIGHT_MOVES, PAWN_MOVES_BLACK, PAWN_MOVES_WHITE}, piece_type::PieceType, square::Square}, moves::slider_gen};

use super::{bit_array::BitArray, bit_array_lookup::{IN_BETWEEN_TABLE, ORTHOGONAL_MOVES}, bit_board::{self, BitBoard}, dynamic_state::DynamicState, field_counter::FieldCounter, piece_type::ColoredPieceType, player_color::PlayerColor};


#[derive(Clone, PartialEq, Eq)]
pub struct AttackBoard {
    white: ColoredAttackBoard<true>,
    black: ColoredAttackBoard<false>,
}

impl AttackBoard {
    pub fn empty() -> AttackBoard {
        AttackBoard {
            white: ColoredAttackBoard::empty(),
            black: ColoredAttackBoard::empty(),
        }
    }

    pub fn attacks(&self, color: PlayerColor) -> u64 {
        if color.is_white() {
            self.white.attacks()
        } else {
            self.black.attacks()
        }
    }

    pub fn add_piece(&mut self, pt: ColoredPieceType, s: i8, bit_board: &BitBoard) {
        if pt.color().is_white() {
            self.white.add_piece(pt, s, bit_board);
        } else {
            self.black.add_piece(pt, s, bit_board);
        }
    }

    pub fn remove_piece(&mut self, pt: ColoredPieceType, s: i8, bit_board: &BitBoard) {
        if pt.color().is_white() {
            self.white.remove_piece(pt, s, bit_board);
        } else {
            self.black.remove_piece(pt, s, bit_board);
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct ColoredAttackBoard<const WHITE: bool> {
    direct: FieldCounter,
    orthogonal: u64,
    diagonal: u64,
}

impl<const WHITE: bool> ColoredAttackBoard<WHITE> {
    pub fn empty() -> ColoredAttackBoard<WHITE> {
        ColoredAttackBoard {
            direct: FieldCounter::empty(),
            orthogonal: 0,
            diagonal: 0,
        }
    }

    pub fn attacks(&self) -> u64 {
        self.direct.attacked() | self.orthogonal | self.diagonal
    }

    pub fn add_piece(&mut self, pt: ColoredPieceType, s: i8, bit_board: &BitBoard) {
        debug_assert!(bit_board.get_piecetype(s) != PieceType::None, "Piece not added");

        let bb = s.bit_array();

        let occupied = bit_board.white_piece | bit_board.black_piece;

        let white_orth_slider = bit_board.orthogonal_slider & bit_board.white_piece;

        if self.orthogonal & bb != 0 || (pt.color().is_white() == WHITE) && pt.is_orthogonal_slider() {
            self.orthogonal = slider_gen::gen_rook_moves_kogge_occ(white_orth_slider, occupied);
        }

        let white_diag_slider = bit_board.diagonal_slider & bit_board.white_piece;
        if self.diagonal & bb != 0 || (pt.color().is_white() == WHITE) && pt.is_diagonal_slider() {
            self.diagonal = slider_gen::gen_bishop_moves_kogge_occ(white_diag_slider, occupied);
        }

        if pt.color().is_white() == WHITE {
            match pt.piece_type() {
                PieceType::Pawn => {
                    if WHITE {
                        self.direct.increment(PAWN_MOVES_WHITE[s as usize]);
                    } else {
                        self.direct.increment(PAWN_MOVES_BLACK[s as usize]);
                    }
                },
                PieceType::Knight => {
                    self.direct.increment(KNIGHT_MOVES[s as usize]);
                },
                PieceType::King => {
                    self.direct.increment(KING_MOVES[s as usize]);
                },
                _ => {},
            };
        }
    }

    pub fn remove_piece(&mut self, pt: ColoredPieceType, s: i8, bit_board: &BitBoard) {
        debug_assert!(bit_board.get_piecetype(s) == PieceType::None, "Piece not removed");

        let bb = s.bit_array();

        let occupied = bit_board.white_piece | bit_board.black_piece;

        let white_orth_slider = bit_board.orthogonal_slider & bit_board.white_piece;

        if self.orthogonal & bb != 0 || (pt.color().is_white() == WHITE) && pt.is_orthogonal_slider() {
            self.orthogonal = slider_gen::gen_rook_moves_kogge_occ(white_orth_slider, occupied);
        }

        let white_diag_slider = bit_board.diagonal_slider & bit_board.white_piece;
        if self.diagonal & bb != 0 || (pt.color().is_white() == WHITE) && pt.is_diagonal_slider() {
            self.diagonal = slider_gen::gen_bishop_moves_kogge_occ(white_diag_slider, occupied);
        }

        if pt.color().is_white() == WHITE {
            match pt.piece_type() {
                PieceType::Pawn => {
                    if WHITE {
                        self.direct.decrement(PAWN_MOVES_WHITE[s as usize]);
                    } else {
                        self.direct.decrement(PAWN_MOVES_BLACK[s as usize]);
                    }
                },
                PieceType::Knight => {
                    self.direct.decrement(KNIGHT_MOVES[s as usize]);
                },
                PieceType::King => {
                    self.direct.decrement(KING_MOVES[s as usize]);
                },
                _ => {},
            };
        }

    }
}