use crate::{board::bit_array_lookup::ROWS, game::board_state::BoardState, moves::move_gen};

use super::settings::Settings;

pub struct Attributes {
    pub piece_count: [i32; 5],
    pub mobility: [i32; 6],
    pub pawn_push: [i32; 6],
}

const PIECE_VALUES: [i32; 5] = [1000, 2800, 3200, 5000, 9000];
const MOBILITY_VALUES: [i32; 6] = [0, 100, 80, 50, 30, 0];
const PAWN_PUSH_VALUE: [i32; 6] = [0, 10, 50, 150, 500, 2000];

pub const STANDARD_EVAL: Attributes = Attributes {
    piece_count: PIECE_VALUES,
    mobility: MOBILITY_VALUES,
    pawn_push: PAWN_PUSH_VALUE,
};

impl Attributes {
    pub fn multiply(&self, weights: &Attributes) -> i32 {
        let mut sum = 0;
        for i in 0..5 {
            sum += self.piece_count[i] * weights.piece_count[i];
        }

        for i in 0..6 {
            sum += self.mobility[i] * weights.mobility[i];
        }

        for i in 0..6 {
            sum += self.pawn_push[i] * weights.pawn_push[i];
        }

        return sum;
    }

    pub fn from_board_state(board_state: &BoardState, setting: &Settings) -> Attributes {
        let mut attributes = Attributes {
            piece_count: [0; 5],
            mobility: [0; 6],
            pawn_push: [0; 6],
        };

        let bb = &board_state.bit_board;
        let white_pawns = bb.white_piece & bb.pawn;
        let black_pawns = bb.black_piece & bb.pawn;

        let white_knights = bb.white_piece & bb.knight;
        let black_knights = bb.black_piece & bb.knight;

        let bishops = bb.diagonal_slider & !bb.orthogonal_slider;
        let white_bishops = bb.white_piece & bishops;
        let black_bishops = bb.black_piece & bishops;

        let rooks = bb.orthogonal_slider & !bb.diagonal_slider;
        let white_rooks = bb.white_piece & rooks;
        let black_rooks = bb.black_piece & rooks;

        let queens = bb.orthogonal_slider & bb.diagonal_slider;
        let white_queens = bb.white_piece & queens;
        let black_queens = bb.black_piece & queens;

        
        attributes.piece_count[0] = 
            white_pawns.count_ones() as i32 - 
            black_pawns.count_ones() as i32;

        attributes.piece_count[1] =
            white_knights.count_ones() as i32 - 
            black_knights.count_ones() as i32;

        attributes.piece_count[2] =
            white_bishops.count_ones() as i32 - 
            black_bishops.count_ones() as i32;

        attributes.piece_count[3] =
            white_rooks.count_ones() as i32 - 
            black_rooks.count_ones() as i32;

        attributes.piece_count[4] =
            white_queens.count_ones() as i32 - 
            black_queens.count_ones() as i32;
        
        let (white_moves, black_moves) = move_gen::gen_eval_moves(&board_state);

        for m in &white_moves {
            attributes.mobility[m.move_piece.piece_type() as usize] += 1;
        }

        for m in &black_moves {
            attributes.mobility[m.move_piece.piece_type() as usize] -= 1;
        }

        if setting.use_pawn_table {
            //Count pawns on rank 
            for i in 0..6 {
                let white_count = (white_pawns & ROWS[i + 1]).count_ones() as i32;
                let black_count = (black_pawns & ROWS[6 - i]).count_ones() as i32;
    
                attributes.pawn_push[i] = white_count - black_count;
            }
        }

        return attributes;
    }
}