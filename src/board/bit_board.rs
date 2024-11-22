use core::panic;

use crate::board::piece_type::PieceType;

use super::{bit_array::BitArray, color::PlayerColor, dynamic_state::DynamicState, piece_type::ColoredPieceType, square::{Square, VALID_SQUARES}};

pub struct BitBoard {
    pub white_piece: BitArray,
    pub black_piece: BitArray,

    pub pawn: BitArray,
    pub knight: BitArray,
    pub diagonal_slider: BitArray,
    pub orthogonal_slider: BitArray, 
    pub king: BitArray,
}

impl BitBoard {
    pub fn empty() -> BitBoard {
        BitBoard {
            white_piece: BitArray::empty(),
            black_piece: BitArray::empty(),
            
            pawn: BitArray::empty(),
            knight: BitArray::empty(),
            diagonal_slider: BitArray::empty(),
            orthogonal_slider: BitArray::empty(),
            king: BitArray::empty(),
        }
    }

    pub fn from_piece_board(piece_board: &crate::board::piece_board::PieceBoard) -> BitBoard {
        let mut bit_board = BitBoard::empty();

        for square in VALID_SQUARES {
            let pt = piece_board[square];
            if pt != ColoredPieceType::None {
                bit_board.add_piece(pt, square);
            }
        }

        bit_board
    }

    pub fn add_piece(&mut self, pt: ColoredPieceType, square: Square) {
        match pt.color() {
            PlayerColor::White => {
                self.white_piece.set_bit(square);
                self.black_piece.clear_bit(square);
            }
            PlayerColor::Black => {
                self.black_piece.set_bit(square);
                self.white_piece.clear_bit(square);
            }
        }

        debug_assert!((self.white_piece & self.black_piece).is_empty());

        match pt.piece_type() {
            PieceType::Pawn => self.pawn.set_bit(square),
            PieceType::Knight => self.knight.set_bit(square),
            PieceType::Bishop => self.diagonal_slider.set_bit(square),
            PieceType::Rook => self.orthogonal_slider.set_bit(square),
            PieceType::Queen => {
                self.diagonal_slider.set_bit(square);
                self.orthogonal_slider.set_bit(square);
            },
            PieceType::King => self.king.set_bit(square),
            PieceType::None => unreachable!()
        }
    }

    pub fn remove_piece(&mut self, pt: ColoredPieceType, square: Square) {
        match pt.color() {
            PlayerColor::White => self.white_piece.clear_bit(square),
            PlayerColor::Black => self.black_piece.clear_bit(square),
        }

        debug_assert!((self.white_piece & self.black_piece).is_empty());

        match pt.piece_type() {
            PieceType::Pawn => self.pawn.clear_bit(square),
            PieceType::Knight => self.knight.clear_bit(square),
            PieceType::Bishop => self.diagonal_slider.clear_bit(square),
            PieceType::Rook => self.orthogonal_slider.clear_bit(square),
            PieceType::Queen => {
                self.diagonal_slider.clear_bit(square);
                self.orthogonal_slider.clear_bit(square);
            },
            PieceType::King => self.king.clear_bit(square),
            PieceType::None => unreachable!()
        }
    }

    //[TODO] Benchmark flip_bit vs ^pos
    pub fn toggle_piece(&mut self, pt: ColoredPieceType, square: Square) {
        match pt.color() {
            PlayerColor::White => self.white_piece.flip_bit(square),
            PlayerColor::Black => self.black_piece.flip_bit(square),
        }

        debug_assert!((self.white_piece & self.black_piece).is_empty());

        match pt.piece_type() {
            PieceType::Pawn => self.pawn.flip_bit(square),
            PieceType::Knight => self.knight.flip_bit(square),
            PieceType::Bishop => self.diagonal_slider.flip_bit(square),
            PieceType::Rook => self.orthogonal_slider.flip_bit(square),
            PieceType::Queen => {
                self.diagonal_slider.flip_bit(square);
                self.orthogonal_slider.flip_bit(square);
            },
            PieceType::King => self.king.flip_bit(square),
            PieceType::None => unreachable!()
        }
    }

    pub fn move_piece(&mut self, pt: ColoredPieceType, start: Square, end: Square) {
        
        debug_assert!(!(self.white_piece | self.black_piece).get_bit(end), "Target square is not empty");
        debug_assert!(pt != ColoredPieceType::None);
        
        let mask = start.bit_array() | end.bit_array();

        match pt.color() {
            PlayerColor::White => self.white_piece ^= mask,
            PlayerColor::Black => self.black_piece ^= mask,
        }

        debug_assert!((self.white_piece & self.black_piece).is_empty());

        match pt.piece_type() {
            PieceType::Pawn => self.pawn ^= mask,
            PieceType::Knight => self.knight ^= mask,
            PieceType::Bishop => self.diagonal_slider ^= mask,
            PieceType::Rook => self.orthogonal_slider ^= mask,
            PieceType::Queen => {
                self.orthogonal_slider ^= mask;
                self.diagonal_slider ^= mask;
            },
            PieceType::King => self.king ^= mask,
            PieceType::None => unreachable!()
        }
    }

    pub fn get_piecetype(&self, square: Square) -> PieceType {
        if !(self.white_piece | self.black_piece).get_bit(square) {
            return PieceType::None;
        }

        if self.pawn.get_bit(square) {
            return PieceType::Pawn;
        }

        if self.knight.get_bit(square) {
            return PieceType::Knight;
        }

        match (self.diagonal_slider.get_bit(square), self.orthogonal_slider.get_bit(square)) {
            (false, false) => {
                debug_assert!(self.king.get_bit(square), "Bitboard is invalid");
                return PieceType::King;
            }
            (true, false) => {
                return PieceType::Bishop;
            }
            (false, true) => {
                return PieceType::Rook;
            }
            (true, true) => {
                return PieceType::Queen;
            }
        }
    }

    pub fn get_colored_piecetype(&self, square: Square) -> ColoredPieceType {
        let c_res = match (self.white_piece.get_bit(square), self.black_piece.get_bit(square)) {
            (false, false) => None,
            (true, false) => Some(PlayerColor::White),
            (false, true) => Some(PlayerColor::Black),
            (true, true) => panic!("Bitboard is invalid"),
        };

        if let Some(color) = c_res {
            return self.get_piecetype(square).colored(color);
        } 

        return ColoredPieceType::None;
    }
}

//[TODO] Implement custom make_move function with toggle piece
impl DynamicState for BitBoard {
    fn empty() -> Self {
        BitBoard::empty()
    }

    fn add_piece(&mut self, pt: ColoredPieceType, s: Square) {
        self.add_piece(pt, s);
    }

    fn remove_piece(&mut self, pt: ColoredPieceType, s: Square) {
        self.remove_piece(pt, s);
    }
}