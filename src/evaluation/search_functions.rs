use std::{collections::HashMap, i32, i64};

use arrayvec::ArrayVec;
use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;

use crate::{
    board::player_color::PlayerColor,
    game::{board_state::BoardState, game_result::GameResult, game_state::GameState},
    moves::{
        chess_move::{self, ChessMove},
        move_gen::{self, MoveVector},
    },
};

use super::{search_stats::SearchStats, settings::Settings};
const MAX_VALUE: i32 = 2_000_000_000;
const CHECKMATE_VALUE: i32 = 1_000_000_000;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum NodeType {
    Exact,
    LowerBound,
    UpperBound,
    Unknown,
}

#[derive(Clone)]
struct TTEntry {
    search_depth: u8,
    score: i32,
    best_move: ChessMove,
    node_type: NodeType,
}

type TranspositionTable = HashMap<u64, TTEntry>;

type QuietMoveTable = [[i64; 64]; 64];

pub fn better_move_sorter(list: &mut MoveVector, board: &BoardState, prev_best: ChessMove) {
    const PIECE_VALUES: [i32; 7] = [10, 28, 32, 50, 90, 100, 0];

    list.sort_by_cached_key(|cm| {
        if *cm == prev_best {
            return i32::MIN;
        }

        let mut sum = if board.bit_board.attacks_king(cm.move_piece, cm.end) {
            500
        } else {
            0
        };
        // let mut sum = 0;

        if cm.is_direct_capture() {
            sum += PIECE_VALUES[cm.captured_piece.piece_type() as usize]
                - PIECE_VALUES[cm.move_piece.piece_type() as usize]
                + 200;
        }

        if cm.is_en_passant() {
            sum += 200;
        }

        sum *= 1000;

        sum += PIECE_VALUES[cm.promotion_piece.piece_type() as usize];

        // sum *= 1000;

        //println!("Move: {} sum: {}", cm.get_board_name(&board), sum);

        return -sum;
    });

    //board.print_local_moves(&list);
}

pub fn quiet_move_sorter(
    list: &mut MoveVector,
    board: &BoardState,
    prev_best: ChessMove,
    quiet_move_table: &QuietMoveTable,
) {
    const PIECE_VALUES: [i64; 7] = [10, 28, 32, 50, 90, 100, 0];

    list.sort_by_cached_key(|cm| {
        if *cm == prev_best {
            return i64::MIN;
        }

        let mut sum = if board.bit_board.attacks_king(cm.move_piece, cm.end) {
            500
        } else {
            0
        };

        if cm.is_direct_capture() {
            sum += PIECE_VALUES[cm.captured_piece.piece_type() as usize]
                - PIECE_VALUES[cm.move_piece.piece_type() as usize]
                + 200;
        }

        if cm.is_en_passant() {
            sum += 200;
        }

        sum *= 1000;

        sum += PIECE_VALUES[cm.promotion_piece.piece_type() as usize];

        sum *= 1_000_000_000;

        if quiet_move_table[cm.start as usize][cm.end as usize] > 1_000_000_000 {
            println!("Overflow in quiet move table");
        }
        sum += quiet_move_table[cm.start as usize][cm.end as usize];

        return -sum;
    });
}

pub fn killer_move_sorter(
    list: &mut MoveVector,
    board: &BoardState,
    prev_best: ChessMove,
    quiet_move_table: &QuietMoveTable,
    killer_move: ChessMove,
) {
    const PIECE_VALUES: [i64; 7] = [10, 28, 32, 50, 90, 100, 0];

    list.sort_by_cached_key(|cm| {
        if *cm == prev_best {
            return i64::MIN;
        }

        let mut sum = if board.bit_board.attacks_king(cm.move_piece, cm.end) {
            500
        } else {
            0
        };

        if cm.is_direct_capture() {
            sum += PIECE_VALUES[cm.captured_piece.piece_type() as usize]
                - PIECE_VALUES[cm.move_piece.piece_type() as usize]
                + 200;
        }

        if cm.is_en_passant() {
            sum += 200;
        }

        sum *= 1000;

        sum += PIECE_VALUES[cm.promotion_piece.piece_type() as usize];

        sum *= 1_000_000_000;

        // if *cm == killer_move {
        //     sum += 100_000;
        // }

        if quiet_move_table[cm.start as usize][cm.end as usize] > 1_000_000_000 {
            println!("Overflow in quiet move table");
        }
        sum += quiet_move_table[cm.start as usize][cm.end as usize];

        return -sum;
    });
}

fn quiessence_search(
    game_state: &mut GameState,
    depth_left: i32,
    depth: i32,
    mut alpha: i32,
    beta: i32,
    settings: &Settings,
    stats: &mut SearchStats,
    tt: &mut TranspositionTable,
    quiet_move_table: &mut QuietMoveTable,
) -> i32 {
    stats.quiessence_nodes += 1;

    let res = game_state.game_result();

    match res {
        GameResult::WhiteWin => return -CHECKMATE_VALUE + depth,
        GameResult::BlackWin => return -CHECKMATE_VALUE + depth,
        GameResult::Draw => return 0,
        GameResult::Undecided => {}
    };

    let res = tt.get(&game_state.zobrist_hash.hash);
    if let Some(entry) = res {
        if entry.search_depth as i32 >= depth_left {
            match entry.node_type {
                NodeType::Exact => return entry.score,
                _ => {}
            }
        }
    }

    let (mut lm, in_check) =
        move_gen::gen_legal_moves_check(&game_state.board_state, &game_state.get_flags());

    if !in_check {
        let factor = match game_state.active_color() {
            PlayerColor::White => 1,
            PlayerColor::Black => -1,
        };
        let local_score = settings.evaluate(game_state) * factor;

        if local_score >= beta {
            return local_score;
        }

        if depth_left <= 0 {
            return local_score;
        }

        alpha = local_score.max(alpha);
    }

    better_move_sorter(&mut lm, &game_state.board_state, chess_move::NULL_MOVE);

    for i in 0..lm.len() {
        let m = lm[i];

        let mut is_quiet_move = !(m.is_capture() || m.is_promotion() || m.is_en_passant());
        if is_quiet_move {
            is_quiet_move = !game_state
                .board_state
                .bit_board
                .attacks_king(m.move_piece, m.end);
        }

        if is_quiet_move && !in_check {
            continue;
        }

        game_state.make_move(m);

        let score = -quiessence_search(
            game_state,
            depth_left - 1,
            depth + 1,
            -beta,
            -alpha,
            settings,
            stats,
            tt,
            quiet_move_table,
        );

        game_state.undo_move();

        if score > alpha {
            alpha = score;

            if alpha >= beta {
                alpha = beta;

                break;
            }
        }
    }

    // if local_score == alpha {
    //     game_state.board_state.piece_board.print();
    // }

    return alpha;
}

pub fn bb_timed_search(
    game_state: &mut GameState,
    time_left: u128,
    settings: &Settings,
) -> (ChessMove, i32, SearchStats) {
    let start_time = std::time::Instant::now();
    let mut stats = SearchStats::new();
    let mut tt = TranspositionTable::new();
    let mut qmt = [[0; 64]; 64];

    let min_time = (time_left as f32 * settings.time_percentage) as u128;
    let mut depth = 1;

    let (eval, last_best_move) = loop {
        let eval = bb_search_settings(
            game_state,
            depth,
            0,
            settings.check_extensions,
            -MAX_VALUE,
            MAX_VALUE,
            settings,
            &mut stats,
            &mut tt,
            &mut qmt,
        );

        let entry = tt.get(&game_state.zobrist_hash.hash).unwrap();
        let best_move = entry.best_move;
        // println!("Depth: {} Best move: {} Score: {}", depth, best_move.to_string(), eval);

        depth += 1;

        if eval.abs() >= CHECKMATE_VALUE || start_time.elapsed().as_millis() > min_time {
            break (eval, best_move);
        }
    };

    // let mut line = Vec::new();

    // // println!("PV line:");
    // for d in 0..1 {
    //     let entry = tt.get(&game_state.zobrist_hash.hash).unwrap();
    //     debug_assert!(entry.node_type == NodeType::Exact);

    //     let best_move = entry.best_move;

    //     // print!("{} ", best_move.to_string());

    //     // println!("Making move: {}", best_move.to_string());
    //     line.push(best_move);
    //     game_state.make_move(best_move);
    // }

    // // println!();

    // for _ in 0..line.len() {
    //     game_state.undo_move();
    // }

    return (last_best_move, eval, stats);
}

fn bb_search_settings(
    game_state: &mut GameState,
    depth_left: i32,
    depth: i32,
    extensions_left: i32,
    mut alpha: i32,
    beta: i32,
    settings: &Settings,
    stats: &mut SearchStats,
    tt: &mut TranspositionTable,
    quiet_move_table: &mut QuietMoveTable,
) -> i32 {
    stats.nodes += 1;

    let res = game_state.game_result();
    match res {
        GameResult::WhiteWin => return -CHECKMATE_VALUE + depth,
        GameResult::BlackWin => return -CHECKMATE_VALUE + depth,
        GameResult::Draw => return 0,
        GameResult::Undecided => (),
    }

    if depth_left == 0 {
        let factor = match game_state.active_color() {
            PlayerColor::White => 1,
            PlayerColor::Black => -1,
        };

        if settings.quiessence_depth == 0 {
            return settings.evaluate(game_state) * factor;
        } else {
            return quiessence_search(
                game_state,
                settings.quiessence_depth,
                depth,
                alpha,
                beta,
                settings,
                stats,
                tt,
                quiet_move_table,
            );
        }

        // let lm = game_state.last_move().unwrap();
    }

    let tt_entry = if tt.contains_key(&game_state.zobrist_hash.hash) {
        tt[&game_state.zobrist_hash.hash].clone()
    } else {
        TTEntry {
            search_depth: 0,
            score: 0,
            best_move: chess_move::NULL_MOVE,
            node_type: NodeType::Unknown,
        }
    };

    if tt_entry.search_depth as i32 == depth_left
        && match tt_entry.node_type {
            NodeType::Exact => true,
            NodeType::LowerBound => tt_entry.score >= beta,
            NodeType::UpperBound => tt_entry.score <= alpha,
            NodeType::Unknown => false,
        }
    {
        // println!("TT hit {:?} Score: {} Alpha {} Beta {}", tt_entry.node_type, tt_entry.score, alpha, beta);
        return tt_entry.score;
    }

    let mut best_move = chess_move::NULL_MOVE;
    let mut best_score = -MAX_VALUE;

    let (mut lm, in_check) = game_state.gen_legal_moves_check();
    let use_extend = in_check && extensions_left > 0 && lm.len() <= 2;

    quiet_move_sorter(
        &mut lm,
        &game_state.board_state,
        tt_entry.best_move,
        quiet_move_table,
    );
    // better_move_sorter(&mut lm, &game_state.board_state, tt_entry.best_move);

    let mut quiets_evaluated: MoveVector = ArrayVec::new();

    let mut node_type = NodeType::UpperBound;
    let mut b = beta;
    for i in 0..lm.len() {
        let m = lm[i];

        let mut is_quiet_move = !(m.is_capture() || m.is_promotion());
        if is_quiet_move {
            is_quiet_move = !game_state
                .board_state
                .bit_board
                .attacks_king(m.move_piece, m.end);
        }

        game_state.make_move(m);

        let mut t = -bb_search_settings(
            game_state,
            depth_left - (!use_extend) as i32,
            depth + 1,
            extensions_left - use_extend as i32,
            -b,
            -alpha,
            settings,
            stats,
            tt,
            quiet_move_table,
        );

        if t > alpha && t < beta && i > 0 && depth_left > 1 {
            t = -bb_search_settings(
                game_state,
                depth_left - (!use_extend) as i32,
                depth + 1,
                extensions_left - use_extend as i32,
                -beta,
                -alpha,
                settings,
                stats,
                tt,
                quiet_move_table,
            );
        }

        game_state.undo_move();

        if t > best_score {
            best_score = t;
            best_move = m;
        }

        if best_score > alpha {
            alpha = best_score;
            node_type = NodeType::Exact;
            if alpha >= beta {
                node_type = NodeType::LowerBound;
                alpha = beta;

                if is_quiet_move {
                    quiet_move_table[m.start as usize][m.end as usize] +=
                        depth_left as i64 * depth_left as i64;

                    for qm in quiets_evaluated {
                        let reduction = depth_left as i64 * depth_left as i64;
                        let val = quiet_move_table[qm.start as usize][qm.end as usize];
                        quiet_move_table[qm.start as usize][qm.end as usize] -= val.min(reduction);
                    }
                }

                break;
            }
        }

        if is_quiet_move {
            quiets_evaluated.push(m);
        }

        b = alpha + 1;
    }

    // println!("Depth: {} Best move: {} Score: {}", depth, best_move.to_string(), alpha);
    // println!("Depth left {}", depth_left);
    // println!("Nodetype: {:?}", node_type);

    if depth_left >= tt_entry.search_depth as i32
        && (node_type == NodeType::Exact
            || tt_entry.node_type == NodeType::Unknown
            || tt_entry.node_type == node_type)
    {
        if best_move.is_null_move() {
            println!("Null move");
            println!("Node type: {:?}", node_type);
            game_state.board_state.piece_board.print();
        }

        tt.insert(
            game_state.zobrist_hash.hash,
            TTEntry {
                search_depth: depth_left as u8,
                score: alpha,
                best_move: best_move,
                node_type: node_type,
            },
        );
    }

    return alpha;
}

pub fn get_random_pos(depth: i32, rng: &mut ChaCha8Rng) -> GameState {
    loop {
        let mut gs = GameState::start_position();

        for i in 0..depth {
            if gs.game_result() != GameResult::Undecided {
                break;
            }

            let moves = gs.gen_legal_moves();
            let m = moves.choose(rng).unwrap();
            gs.make_move(*m);

            if i + 1 == depth {
                return gs;
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use rand::{seq::SliceRandom, Rng, SeedableRng};
//     use rand_chacha::ChaCha8Rng;

//     use super::get_random_pos;

//     #[test]
//     fn test_stable_search() {
//         let mut rng = ChaCha8Rng::seed_from_u64(0);

//         for _ in 0..100 {
//             let depth = rng.gen_range(10..50);
//             let gs = get_random_pos(depth, &mut rng);

//             let (m1, eval1, _) = super::nega_max(&mut gs.clone(), 4);
//             let (m2, eval2, _) = super::nega_alpha_beta(&mut gs.clone(), 4);

//             if eval1 != eval2 {
//                 gs.board_state.piece_board.print();

//                 println!("Depth: {}", depth);
//                 println!("Eval1: {}", eval1);
//                 println!("Eval2: {}", eval2);
//                 m1.print();
//                 m2.print();

//                 panic!();
//             }
//         }
//     }
// }
