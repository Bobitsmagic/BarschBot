use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;

use crate::{board::{bit_board::BitBoard, player_color::PlayerColor}, game::{self, board_state::BoardState, game_result::GameResult, game_state::GameState}, moves::{chess_move::{self, ChessMove}, move_gen::{self, MoveVector}}};

use super::{attributes::{self, Attributes}, search_stats::SearchStats};
const MAX_VALUE: i32 = 2_000_000_000;
const CHECKMATE_VALUE: i32 = 1_000_000_000;

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

pub fn better_move_sorter(list: &mut MoveVector, board: &BitBoard, prev_best: ChessMove) {
    const PIECE_VALUES: [i32; 7] = [10, 28, 32, 50, 90, 100, 0];
     
    list.sort_by_cached_key(|cm| {
        if *cm == prev_best {
            return i32::MIN;
        }
        
        let mut sum = 0;

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

        sum *= 1000;

        // sum += board.get_piece_captures_at(cm.move_piece_type, cm.target_square).iter()
        //     .map(|x| PIECE_VALUES[*x as usize]).sum::<i32>();

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

    better_move_sorter(&mut lm, &game_state.board_state.bit_board, chess_move::NULL_MOVE);

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