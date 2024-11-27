use crate::board::{bit_array::BitArray, bit_array_lookup::{DIAGONAL_MOVES, IN_BETWEEN_TABLE, KNIGHT_MOVES, ORTHOGONAL_MOVES, PAWN_MOVES_BLACK, PAWN_MOVES_WHITE}, bit_board::BitBoard, player_color::PlayerColor};

pub struct CheckPinMask {
    pub check: BitArray,
    pub ortho: BitArray,
    pub diag: BitArray,
}

impl CheckPinMask {
    pub fn pins_on(color: PlayerColor, board: &BitBoard) -> CheckPinMask {
        let opposing_pieces = match color {
            PlayerColor::White => board.black_piece,
            PlayerColor::Black => board.white_piece,
        };
        
        let occupied = board.white_piece | board.black_piece;
        let mut check_mask = BitArray::full();
        let king_square = board.king_position(color);

        //Diagonal slider
        let mut diagonal_mask = BitArray::empty();
        let diagonal_attackers = board.diagonal_slider & opposing_pieces & DIAGONAL_MOVES[king_square as usize];
        for attacker in diagonal_attackers.iterate_set_bits() {
            let between = IN_BETWEEN_TABLE[attacker as usize][king_square as usize];

            match (between & occupied).count_bits() {
                0 => { //Check
                    check_mask &= between | BitArray::new(1_u64 << attacker);
                }
                1 => { //Pin
                    diagonal_mask |= between;
                    diagonal_mask |= BitArray::new(1_u64 << attacker);
                }
                _ => {}
            }
        }

        //Orthogonal slider
        let mut orthogonal_mask = BitArray::empty();
        let orhtogonal_attacker = board.orthogonal_slider & opposing_pieces & ORTHOGONAL_MOVES[king_square as usize];
        for attacker in orhtogonal_attacker.iterate_set_bits() {
            let between = IN_BETWEEN_TABLE[attacker as usize][king_square as usize];
            match (between & occupied).count_bits() {
                0 => { //Check
                    check_mask &= between | BitArray::new(1_u64 << attacker);
                }
                1 => { //Pin
                    orthogonal_mask |= between;
                    orthogonal_mask |= BitArray::new(1_u64 << attacker);
                }
                _ => {}
            }
        }

        //Knights
        let knights= board.knight & opposing_pieces & KNIGHT_MOVES[king_square as usize];
        if !knights.is_empty() {
            debug_assert!(knights.count_bits() != 2);
            
            check_mask &= knights;
        }

        //Pawns
        let pawn_moves = board.pawn & opposing_pieces & match color {
            PlayerColor::White => PAWN_MOVES_WHITE[king_square as usize],
            PlayerColor::Black => PAWN_MOVES_BLACK[king_square as usize],
        };
        
        if !pawn_moves.is_empty() {
            debug_assert!(pawn_moves.count_bits() != 2);

            check_mask &= pawn_moves;
        }


        return CheckPinMask {
            ortho: orthogonal_mask,
            diag: diagonal_mask,
            check: check_mask,
        };
    }
}