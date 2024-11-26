use crate::game::game_state::GameState;

const PERFT_FENS: [&str; 6] = [
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ", 
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ",
    "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ",
    "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
    "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ",
    "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 "
];

pub fn benchmark_fens() {
    const MAX_DEPTH: [u8; 6] = [6, 5, 7, 6, 5, 5];

    let start_time = std::time::Instant::now();
    for p in 0..MAX_DEPTH.len() {
        let fen = PERFT_FENS[p];

        println!("Testing fen: {}", fen);
        let max_depth = MAX_DEPTH[p] + 1;
    
        let d_time = std::time::Instant::now();
            
        let count = count_positions(&mut GameState::from_fen(fen), max_depth);

        println!("Finished depth: {}", max_depth);
        println!("\tTime: {:4.2} s", d_time.elapsed().as_secs_f64());
        println!("\tPos per second: {:.2e}",count as f64 / d_time.elapsed().as_secs_f64());        
    }

    println!("Total time: {}", start_time.elapsed().as_secs_f64());
}

// Small results move gen v1
// Testing fen: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 
// Finished depth: 6
//         Time: 12.43 s
//         Pos per second: 9.58e6
// Testing fen: r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -
// Finished depth: 5
//         Time: 18.81 s
//         Pos per second: 1.03e7
// Testing fen: 8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -
// Finished depth: 7
//         Time: 20.97 s
//         Pos per second: 8.52e6
// Testing fen: r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1
// Finished depth: 6
//         Time: 73.65 s
//         Pos per second: 9.59e6
// Testing fen: rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8
// Finished depth: 5
//         Time: 8.74 s
//         Pos per second: 1.03e7
// Testing fen: r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10
// Finished depth: 5
//         Time: 15.75 s
//         Pos per second: 1.04e7
// Total time: 150.3429325

// Big results move gen v1
// Testing fen: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
// Finished depth: 7
//         Time: 330.26 s
//         Pos per second: 9.68e6
// Testing fen: r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -
// Finished depth: 6
//         Time: 793.66 s
//         Pos per second: 1.01e7
// Testing fen: 8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -
// Finished depth: 8
//         Time: 340.57 s
//         Pos per second: 8.84e6
// Testing fen: r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1
// Finished depth: 7
//         Time: 2868.15 s
//         Pos per second: 9.49e6
// Testing fen: rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8
// Finished depth: 6
//         Time: 297.92 s
//         Pos per second: 1.02e7
// Testing fen: r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10
// Finished depth: 6
//         Time: 645.12 s
//         Pos per second: 1.07e7
// Total time: 5275.6835755

fn count_positions(game_state: &mut GameState, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }
    
    let moves = game_state.gen_legal_moves();
    let mut count = 0;
    for m in moves {
        game_state.make_move(m);
        count += count_positions(game_state, depth - 1);
        game_state.undo_move();
    }

    return count;
}

#[cfg(test)]
mod tests {
    struct PerftRes {
        positions: u64,
        captures: u64,
        en_passants: u64,
        castles: u64,
        promotions: u64,
        checks: u64,
        checkmates: u64,
    }

    use core::panic;
    use std::collections::HashMap;

    use super::*;
    use crate::{fen::fen_helper, game::game_state::GameState, moves::uci_move::UciMove};

    #[test]
    fn test_start_pos_move_types() {
        const POSITIONS: [u64; 9] = [1, 20, 400, 8902, 197281, 4865609, 119060324, 3195901860, 84998978956];
        const CAPTURES: [u64; 9] = [0, 0, 0, 34, 1_576, 82_719, 2_812_008, 108_329_926, 3_523_740_106];
        const CHECKS: [u64; 9] = [0, 0, 0, 12, 469, 27_351, 809_099, 33_103_848, 968_981_593];
        const CHECKMATES: [u64; 9] = [0, 0, 0, 0, 8, 347, 10_828, 435_767, 9_852_036 ];
        const EN_PASSANTS: [u64; 9] = [0, 0, 0, 0, 0, 258, 5248, 319_617, 7_187_977];

        let mut game_state = GameState::start_position();

        for depth in 0..7 {
            let mut res = PerftRes {
                positions: 0,
                captures: 0,
                en_passants: 0,
                castles: 0,
                promotions: 0,
                checks: 0,
                checkmates: 0,
            };


            count_move_types(&mut game_state, depth as u8, &mut res);

            println!("Depth: {}", depth);
            println!("  Positions: {} ({})", res.positions, POSITIONS[depth]);
            println!("  Captures: {} ({})", res.captures, CAPTURES[depth]);
            println!("  Checks: {} ({})", res.checks, CHECKS[depth]);
            println!("  En passants: {} ({})", res.en_passants, EN_PASSANTS[depth]);
            println!("  Checkmates: {} ({})", res.checkmates, CHECKMATES[depth]);

            assert_eq!(res.positions, POSITIONS[depth]);   
            assert_eq!(res.captures, CAPTURES[depth]);
            assert_eq!(res.checks, CHECKS[depth]);
            assert_eq!(res.en_passants, EN_PASSANTS[depth]);
            assert_eq!(res.checkmates, CHECKMATES[depth]);
        }

        fn count_move_types(game_state: &mut GameState, depth: u8, res: &mut PerftRes) {
            if depth == 0 {
                res.positions += 1;
    
                if game_state.board_state.is_in_check(game_state.active_color()) {
                    res.checks += 1;
    
                    if game_state.gen_legal_moves().len() == 0 {
                        res.checkmates += 1;
                    }
                }
    
                return;
            }
            
            let moves = game_state.gen_legal_moves();
            for m in moves {
                if depth == 1 {
                    if m.is_direct_capture() || m.is_en_passant() {
                        res.captures += 1;
                    }
                    if m.is_en_passant() {
                        res.en_passants += 1;
                    }
                    if m.is_castle() {
                        res.castles += 1;
                    }
                    if m.is_promotion() {
                        res.promotions += 1;
                    }
                }
    
                game_state.make_move(m);
                count_move_types(game_state, depth - 1, res);
                game_state.undo_move();
            }
        }
    
    }

    #[test]
    fn test_perft_files() {
        for p in 0..4 {
            let fen = PERFT_FENS[p];

            println!("Testing fen: {}", fen);
            for depth in 0..5 {
                let map = load_perft_file(&format!("data/p{}_perft/perft_{}.txt", p, depth));
    
                check_dfs(&mut GameState::from_fen(fen), depth, &map);
    
                println!("Finished depth: {}", depth);
            }
        }


        fn load_perft_file(path: &str) -> HashMap<String, Vec<UciMove>> {
            let mut map = HashMap::new();
            let contents = std::fs::read_to_string(path).unwrap();

            for line in contents.lines() {
                let parts: Vec<&str> = line.split(",").collect();
                let moves = parts[2].split(" ").filter(|x| *x != "").map(|s| UciMove::from_str(s)).collect();

                // println!("Loading: {}", parts[0]);

                map.insert(parts[0].to_string(), moves);
            }

            return map;
        }
        
        fn check_dfs(game_state: &mut GameState, depth: u8, map: &HashMap<String, Vec<UciMove>>) {
            let mut moves = game_state.gen_legal_moves();

            
            if depth == 0 {

                moves.sort_by(|a, b| a.uci_move().to_string().cmp(&b.uci_move().to_string()));

                let fen = fen_helper::to_fen(&game_state.board_state.piece_board, &game_state.get_flags());

                if !map.contains_key(&fen) {
                    println!("Missing fen: {}", fen);
                    game_state.move_stack.last().unwrap().print();
                    game_state.board_state.piece_board.print();
                    panic!();
                }

                let target_moves = map.get(&fen).unwrap();

                let mut error = moves.len() != target_moves.len();

                if !error {
                    for i in 0..moves.len() {
                        if moves[i].uci_move() != target_moves[i] {
                            error = true;
                            break;
                        }
                    }
                }

                if error {
                    println!("Error at depth: {}", depth);
                    println!("{}", fen);
                    game_state.board_state.piece_board.print();

                    println!("Missing moves: ");
                    for m in target_moves.iter() {
                        let mut found = false;
                        for cm in moves.iter() {
                            if cm.uci_move() == *m {
                                found = true;
                                break;
                            }
                        }

                        if !found {
                            print!("{} ", m.to_string());
                        }
                    }
                    println!();
                    println!("Extra moves: ");
                    for cm in moves.iter() {
                        let mut found = false;
                        for m in target_moves.iter() {
                            if cm.uci_move() == *m {
                                found = true;
                                break;
                            }
                        }

                        if !found {
                            print!("{} ", cm.uci_move().to_string());
                        }
                    }

                    println!();
                    panic!();
                }

                return;
            }

            for m in moves {

                let gs = game_state.clone();
                game_state.make_move(m);
                check_dfs(game_state, depth - 1, map);
                game_state.undo_move();

                if gs != *game_state {
                    println!("Could not undo move");
                    m.print();
                    gs.board_state.piece_board.print();

                    panic!();
                }

            }
        }
    }

    #[test]
    fn test_perft_pos_count() {    
        const RESULTS: [&[u64]; 6] = [
            &[1, 20, 400, 8902, 197281, 4865609, 119060324, 3195901860, 84998978956],
            &[1, 48, 2039, 97862, 4085603, 193690690, 8031647685],
            &[1, 14, 191, 2812, 43238, 674624, 11030083, 178633661, 3009794393],
            &[1, 6, 264, 9467, 422333, 15833292, 706045033],
            &[1, 44, 1486, 62379, 2103487, 89941194],
            &[1, 46, 2079, 89890, 3894594, 164075551, 6923051137],
        ];
    
        for p in 0..RESULTS.len() {
            let fen = PERFT_FENS[p];

            println!("Testing fen: {}", fen);
            for max_depth in 0..RESULTS[p].len() {
                    
                let count = count_positions(&mut GameState::from_fen(fen), max_depth as u8);
                
                assert_eq!(count, RESULTS[p][max_depth as usize]);

                println!("Finished depth: {}", max_depth);
            }
        }
    }

}