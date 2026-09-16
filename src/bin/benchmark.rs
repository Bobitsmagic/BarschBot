use std::{arch::x86_64::_mm_cmp_epu64_mask, time::Instant};

use barschbot::{
    board::{
        bit_array::BitArray, bit_array_lookup::{KING_MOVES, PASSED_PAWN_MASK_BLACK, PASSED_PAWN_MASK_WHITE}, dynamic_state::DynamicState, piece_board::PieceBoard, piece_type::{
            ColoredPieceType::{BlackPawn, WhitePawn},
            PieceType,
        }, rank, square::{self, PAWN_SQUARES, Square, VALID_SQUARES},
    }, evaluation::{
        alpha_beta_search, barschbot::Barschbot, hans_eval::{self, EvaluationSettings, STANDARD_EVAL}, search_functions::{MAX_VALUE, get_random_pos}, settings::Settings, wiesel_eval::{self, WieselSettings},
    }, game::{game_result::GameResult, game_state::GameState}, match_handling, moves::{
        move_gen::{self, gen_king_moves},
        perft_tests::PERFT_FENS,
        slider_gen::{
            gen_bishop_moves, gen_bishop_moves_kogge, gen_bishop_moves_pext, gen_rook_moves,
            gen_rook_moves_kogge, gen_rook_moves_pext,
        },
    },
};

use rand::{rngs::StdRng, Rng};

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    // bench_search_functions();
    // benchmark_fens();
    // passed_pawn_benchmark();
    // gen_king_moves_vs_lookup();
    // compare_eval_functions();
    // slider_gen();
    bench_search_functions();
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

fn gen_king_moves_vs_lookup() {
    const TRY_COUNT: usize = 1 << 24;

    let start_time = Instant::now();
    let mut sum = 0;
    for _ in 0..TRY_COUNT {
        for i in 0..64 {
            let king = 1_u64 << i;
            let square = king.lowest_square_index();

            sum += KING_MOVES[square as usize] & !king;
            sum += square as u64;
        }
    }
    println!("{:?}", start_time.elapsed());
    println!("{}", sum);

    let start_time = Instant::now();
    let mut sum = 0;
    for _ in 0..TRY_COUNT {
        for i in 0..64 {
            let king = 1_u64 << i;
            let square = king.lowest_square_index();

            sum += gen_king_moves(king) & !king;
            sum += square as u64;
        }
    }
    println!("{:?}", start_time.elapsed());
    println!("{}", sum);
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

        let val1 = hans_eval::count_isolated_pawns(white);
        let val2 = hans_eval::count_isolated_kogge(white);

        if val1 != val2 {
            let mut ps = PieceBoard::empty();

            for s in white.iterate_squares() {
                ps.add_piece(WhitePawn, s);
            }

            println!("Isolated: {} {}", val1, val2);
            ps.print();
        }
    }

    println!("Passsed pawns");
    let mut count = 0_i64;
    let start_time = Instant::now();
    for _ in 0..TRY_COUNT {
        for &[white, black] in &pawn_configs {
            let val = hans_eval::count_passed_pawns(white, black, &PASSED_PAWN_MASK_WHITE)
                - hans_eval::count_passed_pawns(black, white, &PASSED_PAWN_MASK_BLACK);

            count += val as i64;
        }
    }
    println!("Count passed: {:?}", start_time.elapsed());
    println!("Count: {}", count);

    let mut count = 0_i64;
    let start_time = Instant::now();
    for _ in 0..TRY_COUNT {
        for &[white, black] in &pawn_configs {
            let val = hans_eval::count_passed_pawns_kogge(white, black);

            count += val as i64;
        }
    }
    println!("CountPassedKogge: {:?}", start_time.elapsed());
    println!("Count: {}", count);
    println!("{}", count / TRY_COUNT as i64);

    //Doubled pawns
    println!("Doubled pawns");
    let mut count = 0_i64;
    let start_time = Instant::now();
    for _ in 0..TRY_COUNT {
        for &[white, black] in &pawn_configs {
            let val = hans_eval::count_doubled_pawns(white) - hans_eval::count_doubled_pawns(black);

            count += val as i64;
        }
    }
    println!("Count doubled: {:?}", start_time.elapsed());
    println!("Count: {}", count);

    let mut count = 0_i64;
    let start_time = Instant::now();
    for _ in 0..TRY_COUNT {
        for &[white, black] in &pawn_configs {
            let val = hans_eval::count_doubled_pawns_kogge(white)
                - hans_eval::count_doubled_pawns_kogge(black);

            count += val as i64;
        }
    }
    println!("Count doubled kogge: {:?}", start_time.elapsed());
    println!("Count: {}", count);
    println!("{}", count / TRY_COUNT as i64);

    //Isolated pawns
    println!("Isolated pawns");
    let mut count = 0_i64;
    let start_time = Instant::now();
    for _ in 0..TRY_COUNT {
        for &[white, black] in &pawn_configs {
            let val =
                hans_eval::count_isolated_pawns(white) - hans_eval::count_isolated_pawns(black);

            count += val as i64;
        }
    }
    println!("Count isolated: {:?}", start_time.elapsed());
    println!("Count: {}", count);

    let mut count = 0_i64;
    let start_time = Instant::now();
    for _ in 0..TRY_COUNT {
        for &[white, black] in &pawn_configs {
            let val =
                hans_eval::count_isolated_kogge(white) - hans_eval::count_isolated_kogge(black);

            count += val as i64;
        }
    }
    println!("Count isolated kogge: {:?}", start_time.elapsed());
    println!("Count: {}", count);
    println!("{}", count / TRY_COUNT as i64);
}

fn compare_eval_functions() {
    let fens = match_handling::file_loader::load_test_fens();

    const TRY_COUNT: usize = 1 << 16;

    let settings = EvaluationSettings {
        use_new_feature: false,
        attr_weights: STANDARD_EVAL,
    };
    let mut sum = 0;
    let start_time = Instant::now();
    for _ in 0..TRY_COUNT {
        for gs in &fens {
            sum += hans_eval::evaluation_function(gs, &settings, -MAX_VALUE, MAX_VALUE)
        }
    }
    println!("Hans false {:?}", start_time.elapsed());
    println!("{}", sum);

    let settings = EvaluationSettings {
        use_new_feature: true,
        attr_weights: STANDARD_EVAL,
    };
    let mut sum = 0;
    let start_time = Instant::now();
    for _ in 0..TRY_COUNT {
        for gs in &fens {
            sum += hans_eval::evaluation_function(gs, &settings, -MAX_VALUE, MAX_VALUE)
        }
    }
    println!("Hans true {:?}", start_time.elapsed());
    println!("{}", sum);

    let settings = WieselSettings {
        pawn_value: 0,
        version: 3,
        piece_weight: [1000, 3000, 3000, 5000, 9000],
    };
    let mut sum = 0;
    let start_time = Instant::now();
    for _ in 0..TRY_COUNT {
        for gs in &fens {
            sum += wiesel_eval::evaluation_function(gs, &settings)
        }
    }
    println!("{:?}", start_time.elapsed());
    println!("{}", sum);
}

pub fn slider_gen() {
    fn fill_board(rng: &mut StdRng) -> (u64, u64) {
        let mut allied = 0;
        let mut opponent = 0;

        for x in 0..8 {
            for y in 0..8 {
                let square = square::from_file_rank(x, y);
                if rng.gen_bool(0.1) {
                    allied.set_bit(square);
                } else if rng.gen_bool(0.1) {
                    opponent.set_bit(square);
                }
            }
        }

        return (allied, opponent);
    }

    const BOARD_COUNT: usize = 1 << 12;
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(0);
    let mut boards = Vec::with_capacity(BOARD_COUNT);
    for _ in 0..BOARD_COUNT {
        boards.push(fill_board(&mut rng));
    }

    const TRY_COUNT: usize = 1 << 10;

    let mut sum = 0;
    let start_time = Instant::now();
    for _ in 0..TRY_COUNT {
        for &(allied, opponent) in &boards {
            for s in VALID_SQUARES {
                let allied = allied | s.bit_array();
                sum += gen_bishop_moves(s, allied, opponent);
            }
        }
    }
    println!("Gen bishop {:?}", start_time.elapsed());
    println!("{}", sum);

    let mut sum = 0;
    let start_time = Instant::now();
    for _ in 0..TRY_COUNT {
        for &(allied, opponent) in &boards {
            for s in VALID_SQUARES {
                let allied = allied | s.bit_array();
                sum += gen_bishop_moves_pext(s, allied | opponent) & !allied;
            }
        }
    }
    println!("Gen bishop pext {:?}", start_time.elapsed());
    println!("{}", sum);

    let mut sum = 0;
    let start_time = Instant::now();
    for _ in 0..TRY_COUNT {
        for &(allied, opponent) in &boards {
            for s in VALID_SQUARES {
                let allied = allied | s.bit_array();
                sum += gen_bishop_moves_kogge(s.bit_array(), allied, opponent);
            }
        }
    }
    println!("Gen bishop kogge {:?}", start_time.elapsed());
    println!("{}", sum);

    println!("Rooks");
    let mut sum = 0;
    let start_time = Instant::now();
    for _ in 0..TRY_COUNT {
        for &(allied, opponent) in &boards {
            for s in VALID_SQUARES {
                let allied = allied | s.bit_array();
                sum += gen_rook_moves(s, allied, opponent);
            }
        }
    }
    println!("Gen rook {:?}", start_time.elapsed());
    println!("{}", sum);

    let mut sum = 0;
    let start_time = Instant::now();
    for _ in 0..TRY_COUNT {
        for &(allied, opponent) in &boards {
            for s in VALID_SQUARES {
                let allied = allied | s.bit_array();
                sum += gen_rook_moves_pext(s, allied | opponent) & !allied;
            }
        }
    }
    println!("Gen rook pext {:?}", start_time.elapsed());
    println!("{}", sum);

    let mut sum = 0;
    let start_time = Instant::now();
    for _ in 0..TRY_COUNT {
        for &(allied, opponent) in &boards {
            for s in VALID_SQUARES {
                let allied = allied | s.bit_array();
                sum += gen_rook_moves_kogge(s.bit_array(), allied, opponent);
            }
        }
    }
    println!("Gen rook kogge {:?}", start_time.elapsed());
    println!("{}", sum);
}

pub fn bench_search_functions() {
    const MAX_DEPTH: i32 = 2;
    const POS_COUNT: usize = 1;
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(37);

    
    let mut bot1 = Barschbot::new(
        Settings {
            time_percentage: 0.02,
            quiessence_depth: 0,
            check_extensions: 0,
            null_move_pruning: 0,
            evaluation_mode: barschbot::evaluation::settings::EvaluationMode::HansEvaluation(
                EvaluationSettings { use_new_feature: false, attr_weights: STANDARD_EVAL }
            )
        }
    );
    
    let mut bot2 = Barschbot::new(
        Settings {
            time_percentage: 0.02,
            quiessence_depth: 0,
            check_extensions: 0,
            null_move_pruning: 1,
            evaluation_mode: barschbot::evaluation::settings::EvaluationMode::HansEvaluation(
                EvaluationSettings { use_new_feature: false, attr_weights: STANDARD_EVAL }
            )
        }
    );
    
    
    let mut positions = Vec::new();
    for i in 0..POS_COUNT {
        let depth = rng.gen_range(50..100);
        
        let gs = loop {
            let gs = get_random_pos(depth, &mut rng);
            
            if matches!(gs.game_result(), GameResult::Undecided) {
               break gs; 
            }
        };
         
        
        positions.push(gs);
    }

    let mut move_list = Vec::new();

    let start_time = Instant::now();
    for i in 0..POS_COUNT {
        let gs = &mut positions[i];
        let cm1 = bot1.search_depth(gs, MAX_DEPTH);
        move_list.push(cm1);
    }
    println!("Bot1: {:?}", start_time.elapsed());

    let mut error_count = 0;
    let start_time = Instant::now();
    for i in 0..POS_COUNT {
        let gs = &mut positions[i];

        // let (cm2, eval2) = alpha_beta_search::go_depth(gs, MAX_DEPTH, &bot2.settings);
        let (cm2, eval2) = bot2.search_depth(gs, MAX_DEPTH);

        let (cm1, eval1) = move_list[i];
        if cm1 != cm2 && eval1 != eval2 {
            println!("{}", i);
            error_count += 1;

            gs.board_state.piece_board.print();
            cm1.print();
            println!("Eval1: {}", eval1);
            cm2.print();
            println!("Eval2: {}", eval2);
            
            println!("{}", gs.to_pgn("White", "Black"));
        }
    }
    println!("Bot2: {:?}", start_time.elapsed());
    println!("ErrorCount: {}", error_count);

}
