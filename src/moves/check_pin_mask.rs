use crate::board::{bit_array::BitArray, bit_array_lookup::{DIAGONAL_MOVES, IN_BETWEEN_TABLE, KNIGHT_MOVES, ORTHOGONAL_MOVES, PAWN_MOVES_BLACK, PAWN_MOVES_WHITE}, bit_board::BitBoard, player_color::PlayerColor, square::{Square, VALID_SQUARES}};

pub struct CheckPinMask {
    pub check: u64,
    pub ortho: u64,
    pub diag: u64,
}

impl CheckPinMask {
    pub fn pins_on(color: PlayerColor, board: &BitBoard) -> CheckPinMask {
        let opposing_pieces = match color {
            PlayerColor::White => board.black_piece,
            PlayerColor::Black => board.white_piece,
        };
        
        let occupied = board.white_piece | board.black_piece;
        let mut check_mask = u64::MAX;
        let king_square = board.king_position(color);

        //Diagonal slider
        let mut diagonal_mask = 0;
        let diagonal_attackers = board.diagonal_slider & opposing_pieces & DIAGONAL_MOVES[king_square as usize];
        for attacker in diagonal_attackers.iterate_set_bits_indices() {
            let between = IN_BETWEEN_TABLE[attacker as usize][king_square as usize];
            
            match (between & occupied).count_ones() {
                0 => { //Check
                    check_mask &= between | (1_u64 << attacker);
                }
                1 => { //Pin
                    diagonal_mask |= between;
                    diagonal_mask.set_bit(VALID_SQUARES[attacker as usize]);
                }
                _ => {}
            }
        }

        //Orthogonal slider
        let mut orthogonal_mask = 0;
        let orhtogonal_attacker = board.orthogonal_slider & opposing_pieces & ORTHOGONAL_MOVES[king_square as usize];
        for attacker in orhtogonal_attacker.iterate_set_bits_indices() {
            let between = IN_BETWEEN_TABLE[attacker as usize][king_square as usize];
            match (between & occupied).count_ones() {
                0 => { //Check
                    check_mask &= between | (1_u64 << attacker);
                }
                1 => { //Pin
                    orthogonal_mask |= between;
                    orthogonal_mask.set_bit(VALID_SQUARES[attacker as usize]);
                }
                _ => {}
            }
        }

        //Knights
        let knights= board.knight & opposing_pieces & KNIGHT_MOVES[king_square as usize];
        if knights != 0 {
            debug_assert!(knights.count_ones() != 2);
            
            check_mask &= knights;
        }

        //Pawns
        let pawn_moves = board.pawn & opposing_pieces & match color {
            PlayerColor::White => PAWN_MOVES_WHITE[king_square as usize],
            PlayerColor::Black => PAWN_MOVES_BLACK[king_square as usize],
        };
        
        if pawn_moves != 0 {
            debug_assert!(pawn_moves.count_ones() != 2);

            check_mask &= pawn_moves;
        }


        return CheckPinMask {
            ortho: orthogonal_mask,
            diag: diagonal_mask,
            check: check_mask,
        };
    }

    pub fn print(&self) {
        println!("Check mask:");
        self.check.print();

        println!("Orthogonal pin mask:");
        self.ortho.print();

        println!("Diagonal pin mask:");
        self.diag.print();
    }
}