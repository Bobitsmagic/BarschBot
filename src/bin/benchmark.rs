use barschbot::{board::{piece_type::PieceType, rank, square::Square}, evaluation::{search_functions::{get_random_pos, nega_alpha_beta, nega_max}, search_stats::SearchStats}, game::game_state::GameState, moves::{move_gen, perft_tests::PERFT_FENS}};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    bench_search_functions();
    // benchmark_fens();
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
        println!("\tPos per second: {:.2e}",count as f64 / d_time.elapsed().as_secs_f64());        
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
    for m in moves.iterate_all_moves(&game_state.board_state.piece_board.clone(), game_state.active_color()) {
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
    for (start, target) in moves.iterate_piece_squares(){
        let m = game_state.board_state.piece_board.get_move(start, target);

        game_state.make_move(m);
        count += count_moves_sperate_iter(game_state, depth - 1);
        game_state.undo_move();
    }

    for (start, target) in moves.iterate_pawn_squares(game_state.active_color()) {
        if target.rank() == rank::R1 || target.rank() == rank::R8 {
            for promotion in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight].iter() {
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

pub fn bench_search_functions() {
    let mut rng = ChaCha8Rng::seed_from_u64(1);

    let mut sum_stats_1 = SearchStats::new();
    let mut sum_stats_2 = SearchStats::new();


    let mut time_1 = 0;
    let mut time_2 = 0;

    for i in 0..100 {
        println!("Iteration: {}", i);

        let depth = rng.gen_range(10..50);
        let gs = get_random_pos(depth, &mut rng);
        
        let start = std::time::Instant::now();
        let (m1, eval1, stats1) = nega_max(&mut gs.clone(), 4);
        time_1 += start.elapsed().as_millis();

        let start = std::time::Instant::now();
        let (m2, eval2, stats2) = nega_alpha_beta(&mut gs.clone(), 4);
        time_2 += start.elapsed().as_millis();
        
        if eval1 != eval2 {
            println!("Different evals");
            println!("  {:?} {:?}", m1, eval1);
            println!("  {:?} {:?}", m2, eval2);
            panic!();
        }

        sum_stats_1.add(&stats1);
        sum_stats_2.add(&stats2);    
    }

    println!("NegaMax:");
    sum_stats_1.print();
    println!("Time: {}", time_1);
    println!("NegaAlphaBeta:");
    sum_stats_2.print();
    println!("Time: {}", time_2);
}