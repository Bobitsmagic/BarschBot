use std::collections::HashMap;

use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;

use crate::{board::{bit_board::BitBoard, player_color::PlayerColor}, game::{self, board_state::BoardState, game_result::GameResult, game_state::GameState}, moves::{chess_move::{self, ChessMove}, move_gen::{self, MoveVector}}};

use super::{attributes::{self, Attributes}, search_stats::SearchStats};
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

pub fn nega_max(game_state: &mut GameState, depth_left: i32) -> (ChessMove, i32, SearchStats) {
    let mut stats = SearchStats::new();
    let (m, eval) = nega_max_search(game_state, depth_left, &mut stats);

    return (m, eval, stats);
}

fn nega_max_search(game_state: &mut GameState, depth_left: i32, stats: &mut SearchStats) -> (ChessMove, i32) {
    stats.nodes += 1;

    let res = game_state.game_result();
    match res {
        GameResult::WhiteWin => return (chess_move::NULL_MOVE, -CHECKMATE_VALUE - depth_left),
        GameResult::BlackWin => return (chess_move::NULL_MOVE, -CHECKMATE_VALUE - depth_left),
        GameResult::Draw => return (chess_move::NULL_MOVE, 0),
        GameResult::Undecided => (),
    }

    if depth_left == 0 {
        stats.evals += 1;

        let factor = match game_state.active_color() {
            PlayerColor::White => 1,
            PlayerColor::Black => -1,
        };
        return (chess_move::NULL_MOVE, factor * Attributes::from_board_state(&game_state.board_state).multiply(&attributes::STANDARD_EVAL));
    }

    let mut best_move = chess_move::NULL_MOVE;
    let mut best_score = -MAX_VALUE;

    for m in move_gen::gen_legal_moves(&game_state.board_state, &game_state.get_flags()) {       
        game_state.make_move(m);
        let (_, score) = nega_max_search(game_state, depth_left - 1, stats);
        game_state.undo_move();

        let score = -score;

        if score > best_score {
            best_score = score;
            best_move = m;
        }
    }

    return (best_move, best_score);
}

pub fn better_move_sorter(list: &mut MoveVector, board: &BoardState, prev_best: ChessMove) {
    const PIECE_VALUES: [i32; 7] = [10, 28, 32, 50, 90, 100, 0];
     
    list.sort_by_cached_key(|cm| {
        if *cm == prev_best {
            return i32::MIN;
        }
        
        let mut sum = if board.bit_board.attacks_king(cm.move_piece, cm.end) { 500 } else { 0 };
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

pub fn nega_alpha_beta(game_state: &mut GameState, max_depth: i32) -> (ChessMove, i32, SearchStats) {
    let mut stats = SearchStats::new();
    let (m, eval) = nega_alpha_beta_search(game_state, max_depth, 0, -MAX_VALUE, MAX_VALUE, &mut stats);

    return (m, eval, stats);
}

fn nega_alpha_beta_search(game_state: &mut GameState, depth_left: i32, depth: i32, mut alpha: i32, beta: i32, stats: &mut SearchStats) -> (ChessMove, i32) {
    stats.nodes += 1;

    let res = game_state.game_result();
    match res {
        GameResult::WhiteWin => return (chess_move::NULL_MOVE, -CHECKMATE_VALUE - depth_left),
        GameResult::BlackWin => return (chess_move::NULL_MOVE, -CHECKMATE_VALUE - depth_left),
        GameResult::Draw => return (chess_move::NULL_MOVE, 0),
        GameResult::Undecided => (),
    }

    if depth_left == 0 {
        stats.evals += 1;

        let factor = match game_state.active_color() {
            PlayerColor::White => 1,
            PlayerColor::Black => -1,
        };
        return (chess_move::NULL_MOVE, factor * Attributes::from_board_state(&game_state.board_state).multiply(&attributes::STANDARD_EVAL));
    }

    let mut best_move = chess_move::NULL_MOVE;
    let mut best_score = -MAX_VALUE;

    let mut lm = game_state.gen_legal_moves();

    better_move_sorter(&mut lm, &game_state.board_state, chess_move::NULL_MOVE);

    for m in lm {       
        game_state.make_move(m);
        let (_, score) = nega_alpha_beta_search(game_state, depth_left - 1, depth + 1, -beta, -alpha, stats);
        game_state.undo_move();

        let score = -score;

        if score > best_score {
            best_score = score;
            best_move = m;

            if best_score > alpha {
                alpha = best_score;
            }
        }

        if alpha >= beta {
            break;
        }
    }

    return (best_move, best_score);
}

pub fn nega_scout(game_state: &mut GameState, max_depth: i32) -> (ChessMove, i32, SearchStats) {
    let mut stats = SearchStats::new();
    let (m, eval) = nega_scout_search(game_state, max_depth, 0, -MAX_VALUE, MAX_VALUE, &mut stats);

    return (m, eval, stats);
}

pub fn nega_scout_search(game_state: &mut GameState, depth_left: i32, depth: i32, mut alpha: i32, beta: i32, stats: &mut SearchStats) -> (ChessMove, i32) {
    stats.nodes += 1;

    let res = game_state.game_result();
    match res {
        GameResult::WhiteWin => return (chess_move::NULL_MOVE, -CHECKMATE_VALUE - depth_left),
        GameResult::BlackWin => return (chess_move::NULL_MOVE, -CHECKMATE_VALUE - depth_left),
        GameResult::Draw => return (chess_move::NULL_MOVE, 0),
        GameResult::Undecided => (),
    }

    if depth_left == 0 {
        stats.evals += 1;

        let factor = match game_state.active_color() {
            PlayerColor::White => 1,
            PlayerColor::Black => -1,
        };
        return (chess_move::NULL_MOVE, factor * Attributes::from_board_state(&game_state.board_state).multiply(&attributes::STANDARD_EVAL));
    }

    let mut best_move = chess_move::NULL_MOVE;

    let mut lm = game_state.gen_legal_moves();

    better_move_sorter(&mut lm, &game_state.board_state, chess_move::NULL_MOVE);

    let mut b = beta;

    for i in 0..lm.len() {       
        let m = lm[i];

        game_state.make_move(m);
        let (_, score) = nega_scout_search(game_state, depth_left - 1, depth + 1, -b, -alpha, stats);
        let mut t = -score;

        if t > alpha && t < beta && i > 0 && depth_left > 1 {
            let (_, score) = nega_scout_search(game_state, depth_left - 1, depth + 1, -beta, -alpha, stats);
            t = -score;
        }
        game_state.undo_move();

        if t >= alpha {
            best_move = m;
            alpha = t;
        }
        

        if alpha >= beta {
            return (best_move, alpha);
        }

        b = alpha + 1;
    }

    return (best_move, alpha);
}

pub fn iterative_deepening(game_state: &mut GameState, max_depth: i32) -> (ChessMove, i32, SearchStats) {
    let mut stats = SearchStats::new();
    let mut best_move = chess_move::NULL_MOVE;
    let mut best_score = -MAX_VALUE;

    for depth in 1..=max_depth {
        let (m, eval, s) = nega_scout(game_state, depth);
        stats.add(&s);

        best_move = m;
        best_score = eval;
    }

    return (best_move, best_score, stats);
}

pub fn nega_alpha_beta_tt(game_state: &mut GameState, max_depth : i32) -> (ChessMove, i32, SearchStats) {
    let mut stats = SearchStats::new();
    let mut tt = TranspositionTable::new();

    let mut best_move = chess_move::NULL_MOVE;
    let mut best_score = -MAX_VALUE;
    for depth in 1..=max_depth {
        let (m, eval) = nega_alpha_beta_tt_search(game_state, depth, 0, -MAX_VALUE, MAX_VALUE, &mut stats, &mut tt);

        best_move = m;
        best_score = eval;
    }
    

    return (best_move, best_score, stats);
}

fn nega_alpha_beta_tt_search(game_state: &mut GameState, depth_left: i32, depth: i32, mut alpha: i32, beta: i32, stats: &mut SearchStats, tt: &mut TranspositionTable) -> (ChessMove, i32) {
    stats.nodes += 1;

    let res = game_state.game_result();
    match res {
        GameResult::WhiteWin => return (chess_move::NULL_MOVE, -CHECKMATE_VALUE - depth_left),
        GameResult::BlackWin => return (chess_move::NULL_MOVE, -CHECKMATE_VALUE - depth_left),
        GameResult::Draw => return (chess_move::NULL_MOVE, 0),
        GameResult::Undecided => (),
    }

    if depth_left == 0 {
        stats.evals += 1;

        let factor = match game_state.active_color() {
            PlayerColor::White => 1,
            PlayerColor::Black => -1,
        };
        return (chess_move::NULL_MOVE, factor * Attributes::from_board_state(&game_state.board_state).multiply(&attributes::STANDARD_EVAL));
    }

    let tt_entry = if tt.contains_key(&game_state.zobrist_hash.hash) {
        tt[&game_state.zobrist_hash.hash].clone()
    }
    else {
        TTEntry {
            search_depth: 0,
            score: 0,
            best_move: chess_move::NULL_MOVE,
            node_type: NodeType::Unknown,
        }
    };

    if tt_entry.search_depth as i32 == depth_left && 
        match tt_entry.node_type {
            NodeType::Exact         => true,
            NodeType::LowerBound    => tt_entry.score >= beta,
            NodeType::UpperBound    => tt_entry.score <= alpha,
            NodeType::Unknown       => false,
        } {
        // println!("TT hit {:?} Score: {} Alpha {} Beta {}", tt_entry.node_type, tt_entry.score, alpha, beta);
        return (tt_entry.best_move, tt_entry.score);
    }

    let mut best_move = chess_move::NULL_MOVE;

    let mut lm = game_state.gen_legal_moves();

    better_move_sorter(&mut lm, &game_state.board_state, tt_entry.best_move);

    let mut node_type = NodeType::UpperBound;
    let mut b = beta;
    for i in 0..lm.len() {       
        let m = lm[i];

        game_state.make_move(m);
        let (_, score) = nega_alpha_beta_tt_search(game_state, depth_left - 1, depth + 1, -b, -alpha, stats, tt);
        let mut t = -score;

        if t > alpha && t < beta && i > 0 && depth_left > 1 {
            let (_, score) = nega_alpha_beta_tt_search(game_state, depth_left - 1, depth + 1, -beta, -alpha, stats, tt);
            t = -score;
        }

        // game_state.board_state.piece_board.print();
        // println!("Score: {}", t);
        game_state.undo_move();

        if t > alpha {
            best_move = m;
            alpha = t;

            // println!("New best move");
            // best_move.print();

            node_type = NodeType::Exact;
            if alpha >= beta {
                // println!("Beta cutoff: {} >= {}", alpha, beta);
                node_type = NodeType::LowerBound;
                best_move = m;
                alpha = beta;
                break;
            }            
        }
        
        b = alpha + 1;
    }

    if depth_left >= tt_entry.search_depth as i32 && (node_type == NodeType::Exact ||
        tt_entry.node_type == NodeType::Unknown || 
        tt_entry.node_type == node_type) {
        
        tt.insert(game_state.zobrist_hash.hash, TTEntry {
            search_depth: depth_left as u8,
            score: alpha,
            best_move: best_move,
            node_type: node_type,
        });
    }

    return (best_move, alpha);
}


pub fn get_random_pos(depth: i32, rng: &mut ChaCha8Rng) -> GameState {
    loop {
        let mut gs = GameState::start_position();

        for i in 0..depth {
            let moves = gs.gen_legal_moves();
            if moves.is_empty() {
                break;
            }

            let m = moves.choose(rng).unwrap();
            gs.make_move(*m);

            if i + 1 == depth {
                return gs;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{seq::SliceRandom, Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    use super::get_random_pos;

    #[test]
    fn test_stable_search() {
        let mut rng = ChaCha8Rng::seed_from_u64(0);

        for _ in 0..100 {
            let depth = rng.gen_range(10..50);
            let gs = get_random_pos(depth, &mut rng);

            
            let (m1, eval1, _) = super::nega_max(&mut gs.clone(), 4);
            let (m2, eval2, _) = super::nega_alpha_beta(&mut gs.clone(), 4);
            
            if eval1 != eval2 {
                gs.board_state.piece_board.print();

                println!("Depth: {}", depth);
                println!("Eval1: {}", eval1);
                println!("Eval2: {}", eval2);
                m1.print();
                m2.print();

                panic!();
            }
        }
    }
}