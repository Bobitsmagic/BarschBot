use crate::{
    board::{
        bit_array::BitArray,
        bit_array_lookup::{self, ACCUM_COLUMNS, COLUMNS, ROWS},
        piece_type::ColoredPieceType::BlackPawn,
    },
    evaluation::settings::EvaluationMode::HansEvaluation,
    game::game_state::GameState,
    moves::move_gen,
};

use crate::board::square::Square;

#[derive(Debug, Clone, Copy)]
pub struct Attributes {
    pub piece_weight: [i32; 5],
    pub mobility: [i32; 6],
    pub pawn_push: [i32; 6],
    pub double_pawn: i32,
    pub isolated_pawn: i32,
    pub passed_pawn: i32,
    pub turn: i32,
    pub king_border_distance: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct EvaluationSettings {
    pub use_new_feature: bool,
    pub attr_weights: Attributes,
}

pub const STANDARD_EVAL: Attributes = Attributes {
    piece_weight: [1000, 3200, 3300, 5000, 9000],
    mobility: [0, 50, 40, 30, 20, 0],
    pawn_push: [0, 10, 50, 150, 500, 2000],
    passed_pawn: 150,
    double_pawn: -20,
    isolated_pawn: -10,
    turn: 1,
    king_border_distance: 1,
};

pub fn evaluation_function(gs: &GameState, eval_settings: &EvaluationSettings) -> i32 {
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

    let attr = &eval_settings.attr_weights;
    let mut sum = 0;
    sum += (white_pawns.count_ones() as i32 - black_pawns.count_ones() as i32) * attr.piece_weight[0];
    sum += (white_knights.count_ones() as i32 - black_knights.count_ones() as i32) * attr.piece_weight[1];
    sum += (white_bishops.count_ones() as i32 - black_bishops.count_ones() as i32) * attr.piece_weight[2];
    sum += (white_rooks.count_ones() as i32 - black_rooks.count_ones() as i32) * attr.piece_weight[3];
    sum += (white_queens.count_ones() as i32 - black_queens.count_ones() as i32) * attr.piece_weight[4];

    let mobi = move_gen::count_eval_moves(board_state);

    for i in 0..mobi.len() {
        sum += mobi[i] * attr.mobility[i];
    }

    //Count pawns on rank
    for i in 0..6 {
        let white_count = (white_pawns & ROWS[i + 1]).count_ones() as i32;
        let black_count = (black_pawns & ROWS[6 - i]).count_ones() as i32;

        sum += (white_count - black_count) * attr.pawn_push[i];
    }

    if (bb.white_piece | bb.black_piece).count_ones() == 3
        && bb.orthogonal_slider.count_ones() == 1
    {
        let w_square = (bb.king & bb.white_piece).trailing_zeros() as i8;
        let b_square = (bb.king & bb.black_piece).trailing_zeros() as i8;
        let w_dist = w_square.rank().min(7 - w_square.rank())
            + (w_square.file().min(7 - w_square.file()));
        let b_dist = b_square.rank().min(7 - b_square.rank())
            + (b_square.file().min(7 - b_square.file()));

        sum += w_dist as i32 - b_dist as i32;
    }

    //Pawn eval
    if eval_settings.use_new_feature {
        sum += count_passed_pawns_kogge(white_pawns, black_pawns) * attr.passed_pawn;
    }

    return sum;
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
        let file = bit_array_lookup::COLUMNS[x] & pawns;
        let isolated = (bit_array_lookup::ADJACENT_COLUMNS[x] & pawns) == 0;

        isolated_pawns += (file.count_ones() as i32) * isolated as i32;
    }

    return isolated_pawns;
}

pub fn count_passed_pawns(allied_pawns: u64, enemy_pawns: u64, pawn_mask: &[u64; 64]) -> i32 {
    let mut passed_pawns = 0;

    for s in allied_pawns.iterate_set_bits_indices() {
        passed_pawns += ((enemy_pawns & pawn_mask[s as usize]) == 0) as i32;
    }

    return passed_pawns;
}

pub fn count_passed_pawns_kogge(white_pawn: u64, black_pawn: u64) -> i32 {
    let mut black_wall =
        (black_pawn | (black_pawn << 1) & !COLUMNS[0] | (black_pawn >> 1) & !COLUMNS[7]) >> 8;
    let mut white_wall =
        (white_pawn | (white_pawn << 1) & !COLUMNS[0] | (white_pawn >> 1) & !COLUMNS[7]) << 8;

    black_wall |= black_wall >> 8;
    black_wall |= black_wall >> 16;
    black_wall |= black_wall >> 32;

    white_wall |= white_wall << 8;
    white_wall |= white_wall << 16;
    white_wall |= white_wall << 32;

    return (white_pawn & !black_wall).count_ones() as i32
        - (black_pawn & !white_wall).count_ones() as i32;
}

#[test]
fn check_board_symmetry() {
    let fens = crate::match_handling::file_loader::load_test_fens();

    for gs in fens {
        let v1 = evaluation_function(
            &gs,
            &EvaluationSettings {
                use_new_feature: true,
                attr_weights: STANDARD_EVAL,
            },
        );

        let v2 = evaluation_function(
            &gs.fliped_state(),
            &EvaluationSettings {
                use_new_feature: true,
                attr_weights: STANDARD_EVAL,
            },
        );

        assert!(v1 == -v2)
    }
}
