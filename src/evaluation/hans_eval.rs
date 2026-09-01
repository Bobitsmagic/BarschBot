use crate::{board::{self, bit_array::BitArray, bit_array_lookup::{self, ROWS}, player_color::PlayerColor}, game::{board_state::BoardState, game_state::GameState}, moves::move_gen};
use super::settings::Settings;
use crate::board::bit_array;

#[derive(Debug, Clone, Copy)]
pub struct Attributes {
    pub piece_count: [i32; 5],
    pub mobility: [i32; 6],
    pub pawn_push: [i32; 6],
    pub double_pawn: i32,
    pub isolated_pawn: i32,
    pub passed_pawn: i32,
    pub turn: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct EvaluationSettings {
    pub use_new_feature: bool,
    pub attr_weights: Attributes   
}

pub fn evaluation_function(gs: &GameState, eval_settings: &EvaluationSettings) -> i32 {
    let attr = Attributes::from_board_state(gs, eval_settings);

    return  attr.multiply(&eval_settings.attr_weights);
}

const PIECE_VALUES: [i32; 5] = [1000, 2800, 3200, 5000, 9000];
const MOBILITY_VALUES: [i32; 6] = [0, 100, 80, 50, 30, 0];
const PAWN_PUSH_VALUE: [i32; 6] = [0, 10, 50, 150, 500, 2000];
const DOUBLE_PAWN_VALUE: i32 = -200;
const ISOLATED_PAWN_VALUE: i32 = -100;
const PASSED_PAWN_VALUE: i32 = 150;
const TURN_VALUE: i32 = 20;

pub const STANDARD_EVAL: Attributes = Attributes {
    piece_count: PIECE_VALUES,
    mobility: MOBILITY_VALUES,
    pawn_push: PAWN_PUSH_VALUE,
    double_pawn: DOUBLE_PAWN_VALUE,
    isolated_pawn: ISOLATED_PAWN_VALUE,
    passed_pawn: PASSED_PAWN_VALUE,
    turn: TURN_VALUE,
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

        sum += self.double_pawn * weights.double_pawn;
        sum += self.isolated_pawn * weights.isolated_pawn;
        sum += self.passed_pawn * weights.passed_pawn;

        return sum;
    }

    pub fn from_board_state(gs: &GameState, setting: &EvaluationSettings) -> Attributes {
        let mut attributes = Attributes {
            piece_count: [0; 5],
            mobility: [0; 6],
            pawn_push: [0; 6],
            double_pawn: 0,
            isolated_pawn: 0,
            passed_pawn: 0,
            turn: 0,
        };

        let board_state = &gs.board_state;
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
        
        //Count pawns on rank 
        for i in 0..6 {
            let white_count = (white_pawns & ROWS[i + 1]).count_ones() as i32;
            let black_count = (black_pawns & ROWS[6 - i]).count_ones() as i32;

            attributes.pawn_push[i] = white_count - black_count;
        }

        attributes.double_pawn = count_doubled_pawns(white_pawns) - count_doubled_pawns(black_pawns);
        
        if setting.use_new_feature {
            attributes.isolated_pawn = count_isolated_pawns(white_pawns) - count_isolated_pawns(black_pawns);
            attributes.passed_pawn = count_passed_pawns(white_pawns, black_pawns, &bit_array_lookup::PASSED_PAWN_MASK_WHITE);
            attributes.passed_pawn -= count_passed_pawns(black_pawns, white_pawns, &bit_array_lookup::PASSED_PAWN_MASK_BLACK);

            attributes.turn = match gs.active_color() {
                PlayerColor::White => 1,
                PlayerColor::Black => -1,
            };
        }

        return attributes;
    }
}

fn count_doubled_pawns(pawns: u64) -> i32 {
    let mut doubled_pawns = 0;

    for i in 1..=5 {
        let count = (pawns & pawns.translate_vertical(i)).count_ones();
        doubled_pawns += count;
    }

    return doubled_pawns as i32;
}

fn count_isolated_pawns(pawns: u64) -> i32 {
    let mut isolated_pawns = 0;
    for x in 0..8 {
        let file = bit_array_lookup::COLLUMNS[x] & pawns;
        let isolated = (bit_array_lookup::ADJACENT_COLUMNS[x] & pawns) == 0;

        isolated_pawns += (file.count_ones() as i32) * isolated as i32;
    }

    return isolated_pawns;
}

fn count_passed_pawns(allied_pawns: u64, enemy_pawns: u64, pawn_mask: &[u64; 64]) -> i32 {
    let mut passed_pawns = 0;

    for s in allied_pawns.iterate_set_bits_indices() {
        passed_pawns += ((enemy_pawns & pawn_mask[s as usize]) == 0) as i32;
    }

    return passed_pawns;
}
