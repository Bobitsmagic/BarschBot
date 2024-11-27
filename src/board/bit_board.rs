use core::panic;

use crate::board::piece_type::PieceType;

use super::{bit_array::BitArray, bit_array_lookup::{DIAGONAL_MOVES, IN_BETWEEN_TABLE, KING_MOVES, KNIGHT_MOVES, ORTHOGONAL_MOVES}, dynamic_state::DynamicState, piece_type::ColoredPieceType, player_color::PlayerColor, square::{Square, VALID_SQUARES}};

#[derive(Clone, PartialEq, Eq)]
pub struct BitBoard {
    pub white_piece: u64,
    pub black_piece: u64,

    pub pawn: u64,
    pub knight: u64,
    pub diagonal_slider: u64,
    pub orthogonal_slider: u64, 
    pub king: u64,
}

impl BitBoard {
    pub fn empty() -> BitBoard {
        BitBoard {
            white_piece: 0,
            black_piece: 0,
            
            pawn: 0,
            knight: 0,
            diagonal_slider: 0,
            orthogonal_slider: 0,
            king: 0,
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

    pub fn pawn_has_neighbour(&self, color: PlayerColor, square: Square) -> bool {
        let opponents = match color {
            PlayerColor::White => self.black_piece,
            PlayerColor::Black => self.white_piece,
        };

        let mask = square.bit_array();

        return self.pawn & opponents & (mask.translate(1, 0) | mask.translate(-1, 0)) != 0;
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

        debug_assert!((self.white_piece & self.black_piece) == 0);

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

        debug_assert!((self.white_piece & self.black_piece) == 0);

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

        debug_assert!((self.white_piece & self.black_piece) == 0);

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

        debug_assert!((self.white_piece & self.black_piece) == 0);

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

    pub fn king_position(&self, color: PlayerColor) -> Square {
        let color = match color {
            PlayerColor::White => self.white_piece,
            PlayerColor::Black => self.black_piece,
        };

        (color & self.king).iterate_squares().next().unwrap()
    }

    pub fn square_is_attacked_through_king(&self, target_square: Square, attacker_color: PlayerColor) -> bool {
        let opponent = match attacker_color {
            PlayerColor::White => self.white_piece,
            PlayerColor::Black => self.black_piece,
        };
        let allied = match attacker_color {
            PlayerColor::White => self.black_piece,
            PlayerColor::Black => self.white_piece,
        };

        let occupied = (self.white_piece | self.black_piece) & !(self.king & allied);

        // println!("Occupied:");
        // occupied.print();
        
        //Knights
        if (self.knight & opponent & KNIGHT_MOVES[target_square as usize]) != 0 {
            return true;
        }

        // println!("King square: {:?}", king_square);
        //Sliders
        let diagonal_sliders = self.diagonal_slider & opponent & DIAGONAL_MOVES[target_square as usize];
        for attacker in diagonal_sliders.iterate_squares() {
            let between = IN_BETWEEN_TABLE[attacker as usize][target_square as usize];
            if (between & occupied) == 0 {
                return true;
            }
        }
        let orthogonal_sliders = self.orthogonal_slider & opponent & ORTHOGONAL_MOVES[target_square as usize];
        for attacker in orthogonal_sliders.iterate_squares() {
            let between = IN_BETWEEN_TABLE[attacker as usize][target_square as usize];
            if (between & occupied) == 0 {
                return true;
            }
        }

        //Pawns
        let dy = match attacker_color {
            PlayerColor::White => -1,
            PlayerColor::Black => 1,
        };

        let king_bit = target_square.bit_array();
        let king_pawn_attack = king_bit.translate(-1, dy) | king_bit.translate(1, dy);
        if (self.pawn & opponent & king_pawn_attack) != 0 {
            return true;
        }
        
        //King
        let king_moves = KING_MOVES[target_square as usize];
        if (self.king & opponent & king_moves) != 0 {
            return true;
        }

        return false;
    }

    pub fn square_is_attacked_by(&self, target_square: Square, opponent_color: PlayerColor) -> bool {
        let opponent = match opponent_color {
            PlayerColor::White => self.white_piece,
            PlayerColor::Black => self.black_piece,
        };

        let occupied = self.white_piece | self.black_piece;

        //Knights
        if (self.knight & opponent & KNIGHT_MOVES[target_square as usize]) != 0 {
            return true;
        }

        // println!("King square: {:?}", king_square);
        //Sliders
        let diagonal_sliders = self.diagonal_slider & opponent & DIAGONAL_MOVES[target_square as usize];
        for attacker in diagonal_sliders.iterate_squares() {
            let between = IN_BETWEEN_TABLE[attacker as usize][target_square as usize];
            if (between & occupied) == 0 {
                return true;
            }
        }
        let orthogonal_sliders = self.orthogonal_slider & opponent & ORTHOGONAL_MOVES[target_square as usize];
        for attacker in orthogonal_sliders.iterate_squares() {
            let between = IN_BETWEEN_TABLE[attacker as usize][target_square as usize];
            if (between & occupied) == 0 {
                return true;
            }
        }

        //Pawns
        let dy = match opponent_color {
            PlayerColor::White => -1,
            PlayerColor::Black => 1,
        };

        let king_bit = target_square.bit_array();
        let king_pawn_attack = king_bit.translate(-1, dy) | king_bit.translate(1, dy);
        if (self.pawn & opponent & king_pawn_attack) != 0 {
            return true;
        }
        
        //King
        let king_moves = KING_MOVES[target_square as usize];
        if (self.king & opponent & king_moves) != 0 {
            return true;
        }

        return false;
    }
    pub fn is_in_check(&self, color: PlayerColor) -> bool {
        return self.square_is_attacked_by(self.king_position(color), !color);
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