use crate::board::{bit_array::BitArray, bit_array_lookup::{DIAGONAL_MOVES, IN_BETWEEN_TABLE, ORTHOGONAL_MOVES}, bit_board::BitBoard, player_color::PlayerColor};

pub struct PinMask {
    pub orthogonal_mask: BitArray,
    pub diagonal_mask: BitArray,
}

impl PinMask {
    pub fn empty() -> PinMask {
        PinMask {
            orthogonal_mask: BitArray::empty(),
            diagonal_mask: BitArray::empty(),
        }
    }

    pub fn pins_on(color: PlayerColor, board: &BitBoard) -> PinMask {
        let opposing_pieces = match color {
            PlayerColor::White => board.black_piece,
            PlayerColor::Black => board.white_piece,
        };

        let allied_pieces = match color {
            PlayerColor::White => board.white_piece,
            PlayerColor::Black => board.black_piece,
        };

        let occupied = board.white_piece | board.black_piece;
        let king_square = board.king_position(color);

        let mut diagonal_mask = BitArray::empty();
        let diagonal_attackers = board.diagonal_slider & opposing_pieces & DIAGONAL_MOVES[king_square as usize];
        for attacker in diagonal_attackers.iterate_set_bits() {
            let between = IN_BETWEEN_TABLE[attacker as usize][king_square as usize];
            if (between & occupied).count_bits() == 1 &&
               (between & allied_pieces).count_bits() == 1 { 
                diagonal_mask |= between;
            }
        }

        let mut orthogonal_mask = BitArray::empty();
        let orhtogonal_attacker = board.orthogonal_slider & opposing_pieces & ORTHOGONAL_MOVES[king_square as usize];
        for attacker in orhtogonal_attacker.iterate_set_bits() {
            let between = IN_BETWEEN_TABLE[attacker as usize][king_square as usize];
            if (between & occupied).count_bits() == 1 &&
               (between & allied_pieces).count_bits() == 1 { 
                orthogonal_mask |= between;
            }
        }

        return PinMask {
            orthogonal_mask,
            diagonal_mask,
        };
    }
}