use crate::board::piece_type::PieceType;

use super::{bit_array::BitArray, color::Color, piece_type::ColoredPieceType, square::Square};

pub struct BitBoard {
    white_piece: BitArray,
    black_piece: BitArray,

    pawn: BitArray,
    knight: BitArray,
    diagonal_slider: BitArray,
    orthogonal_slider: BitArray, 
    king: BitArray,
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

    pub fn set_piece(&mut self, pt: ColoredPieceType, square: Square) {
        match pt.color() {
            Color::White => {
                self.white_piece.set_bit(square);
                self.black_piece.clear_bit(square);
            }
            Color::Black => {
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
            Color::White => self.white_piece.clear_bit(square),
            Color::Black => self.black_piece.clear_bit(square),
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
            Color::White => self.white_piece.flip_bit(square),
            Color::Black => self.black_piece.flip_bit(square),
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
        
        let mask = start.get_bitarray() | end.get_bitarray();

        match pt.color() {
            Color::White => self.white_piece ^= mask,
            Color::Black => self.black_piece ^= mask,
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

}