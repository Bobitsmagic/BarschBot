use std::time::Instant;

use barschbot::{
    board::{
        bit_array::BitArray,
        bit_array_lookup::{PASSED_PAWN_MASK_BLACK, PASSED_PAWN_MASK_WHITE},
        dynamic_state::DynamicState,
        piece_board::PieceBoard,
        piece_type::{
            ColoredPieceType::{BlackPawn, WhitePawn},
            PieceType,
        },
        rank,
        square::{self, Square, PAWN_SQUARES},
    },
    evaluation::{hans_eval, search_stats::SearchStats},
    game::{
        board_state::{self, BoardState},
        game_state::GameState,
    },
    moves::{chess_move::ChessMove, move_gen, perft_tests::PERFT_FENS},
};

use piston_window::math::square_len;
use rand::{rngs::StdRng, Rng};

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    // bench_search_functions();
    // benchmark_fens();
    passed_pawn_benchmark();
}

pub fn benchmark_fens() {
    const MAX_DEPTH: [u8; 6] = [6, 5, 7, 6, 5, 5];

    let start_time = std::time::Instant::now();
    for p in 0..MAX_DEPTH.len() {
        let fen = PERFT_FENS[p];

        println!("Testing fen: {}", fen);
        let max_depth = MAX_DEPTH[p] + 1;

        let d_time = std::time::Instant::now();

        let count = count_moves(&mut GameState::from_fen(fen), max_depth);
        // let count = count_moves_iter(&mut GameState::from_fen(fen), max_depth);
        // let count = count_moves_sperate_iter(&mut GameState::from_fen(fen), max_depth);

        println!("Finished depth: {}", max_depth);
        println!("\tTime: {:4.2} s", d_time.elapsed().as_secs_f64());
        println!(
            "\tPos per second: {:.2e}",
            count as f64 / d_time.elapsed().as_secs_f64()
        );
    }

    println!("Total time: {}", start_time.elapsed().as_secs_f64());
}

fn count_moves_iter(game_state: &mut GameState, depth: u8) -> u64 {
    // moves.print();

    if depth == 0 {
        return 1;
    }
    // if depth == 1 {
    //     return moves.count_moves() as u64;
    // }

    let moves = game_state.gen_legal_moves_iterator();
    let mut count = 0;
    for m in moves.iterate_all_moves(
        &game_state.board_state.piece_board.clone(),
        game_state.active_color(),
    ) {
        game_state.make_move(m);
        count += count_moves_iter(game_state, depth - 1);
        game_state.undo_move();
    }

    return count;
}

fn count_moves_sperate_iter(game_state: &mut GameState, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    if depth == 1 {
        return move_gen::count_moves(&game_state.board_state, &game_state.get_flags()) as u64;
    }

    let moves = game_state.gen_legal_moves_iterator();
    let mut count = 0;
    for (start, target) in moves.iterate_piece_squares() {
        let m = game_state.board_state.piece_board.get_move(start, target);

        game_state.make_move(m);
        count += count_moves_sperate_iter(game_state, depth - 1);
        game_state.undo_move();
    }

    for (start, target) in moves.iterate_pawn_squares(game_state.active_color()) {
        if target.rank() == rank::R1 || target.rank() == rank::R8 {
            for promotion in [
                PieceType::Queen,
                PieceType::Rook,
                PieceType::Bishop,
                PieceType::Knight,
            ]
            .iter()
            {
                let mut m = game_state.board_state.piece_board.get_move(start, target);
                m.promotion_piece = promotion.colored(game_state.active_color());

                game_state.make_move(m);
                count += count_moves_sperate_iter(game_state, depth - 1);
                game_state.undo_move();
            }
        } else {
            let m = game_state.board_state.piece_board.get_move(start, target);

            game_state.make_move(m);
            count += count_moves_sperate_iter(game_state, depth - 1);
            game_state.undo_move();
        }
    }

    return count;
}

fn count_moves(game_state: &mut GameState, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    if depth == 1 {
        return move_gen::count_moves(&game_state.board_state, &game_state.get_flags()) as u64;
    }

    let moves = game_state.gen_legal_moves();
    let mut count = 0;
    for m in moves {
        game_state.make_move(m);
        count += count_moves(game_state, depth - 1);
        game_state.undo_move();
    }

    return count;
}

fn passed_pawn_benchmark() {
    const CONFIG_SIZE: usize = 1 << 10;
    const TRY_COUNT: usize = 1 << 20;
    let mut pawn_configs = Vec::new();

    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(1);

    let square_list = PAWN_SQUARES.collect::<Vec<_>>();
    for _ in 0..CONFIG_SIZE {
        let mut list = square_list.clone();

        let pawn_count_white = rng.gen_range(0..8);
        let mut white = 0_u64;
        for _ in 0..pawn_count_white {
            let index = rng.gen_range(0..list.len());
            let val = list.remove(index);
            white |= val.bit_array();
        }

        let pawn_count_black = rng.gen_range(0..8);
        let mut black = 0_u64;
        for _ in 0..pawn_count_black {
            let index = rng.gen_range(0..list.len());
            let val = list.remove(index);
            black |= val.bit_array();
        }

        pawn_configs.push([white, black]);

        let val1 = hans_eval::count_passed_pawns(white, black, &PASSED_PAWN_MASK_WHITE)
            - hans_eval::count_passed_pawns(black, white, &PASSED_PAWN_MASK_BLACK);

        let val2 = hans_eval::count_passed_pawns_kogge(white, black);

        if val1 != val2 {
            let mut ps = PieceBoard::empty();

            for s in white.iterate_squares() {
                ps.add_piece(WhitePawn, s);

                println!("{}", s.square_string());
                PASSED_PAWN_MASK_WHITE[s as usize].print();
            }

            for s in black.iterate_squares() {
                ps.add_piece(BlackPawn, s);
                println!("{}", s.square_string());
                PASSED_PAWN_MASK_BLACK[s as usize].print();
            }

            println!("{} {}", val1, val2);
            ps.print();
        }
    }

    let mut count = 0_i64;
    let start_time = Instant::now();
    for _ in 0..TRY_COUNT {
        for &[white, black] in &pawn_configs {
            let val = hans_eval::count_passed_pawns(white, black, &PASSED_PAWN_MASK_WHITE)
                - hans_eval::count_passed_pawns(black, white, &PASSED_PAWN_MASK_BLACK);

            count += val as i64;
        }
    }
    println!("Old: {:?}", start_time.elapsed());
    println!("Count: {}", count);

    let mut count = 0_i64;
    let start_time = Instant::now();
    for _ in 0..TRY_COUNT {
        for &[white, black] in &pawn_configs {
            let val = hans_eval::count_passed_pawns_kogge(white, black);

            count += val as i64;
        }
    }
    println!("Old: {:?}", start_time.elapsed());
    println!("Count: {}", count);
    println!("{}", count / TRY_COUNT as i64)
}

// pub fn bench_search_functions() {
//     const MAX_DEPTH: i32 = 7;

//     let mut rng = ChaCha8Rng::seed_from_u64(2);

//     const FUNCTIONS: [fn(&mut GameState, i32) -> (ChessMove, i32, SearchStats); 3] = [nega_alpha_beta_tt, nega_alpha_beta_tt_qmt, aspiration_window];

//     let mut sum_stats = Vec::new();
//     let mut times = vec![0; FUNCTIONS.len()];

//     for _ in 0..FUNCTIONS.len() {
//         sum_stats.push(SearchStats::new());
//     }

//     for i in 0..100 {
//         println!("Iteration: {}", i);

//         let depth = rng.gen_range(10..50);
//         let gs = get_random_pos(depth, &mut rng);

//         let mut evals = Vec::new();
//         for j in 0..FUNCTIONS.len() {
//             let start = std::time::Instant::now();
//             let (_, eval, stats) = FUNCTIONS[j](&mut gs.clone(), MAX_DEPTH);
//             times[j] += start.elapsed().as_millis();
//             sum_stats[j] += stats;
//             evals.push(eval);
//         }

//         for j in 1..FUNCTIONS.len() {
//             if evals[j] != evals[0] {
//                 println!("Different move found!");
//                 gs.board_state.piece_board.print();
//                 println!("index: {}", j);
//                 println!("Evals: {} {}", evals[0], evals[j]);
//                 panic!();
//             }
//         }
//     }

//     for i in 0..FUNCTIONS.len() {
//         println!("Function: {}", i);
//         println!("Time: {} ms", times[i]);
//         sum_stats[i].print();
//     }
// }
