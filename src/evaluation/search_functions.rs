use crate::{board::player_color::PlayerColor, game::{self, board_state::BoardState, game_result::GameResult, game_state::GameState}, moves::{chess_move::{self, ChessMove}, move_gen}};

use super::attributes::{self, Attributes};
const MAX_VALUE: i32 = 2_000_000_000;
const CHECKMATE_VALUE: i32 = 1_000_000_000;

pub fn negamax(game_state: &mut GameState, depth_left: i32) -> (ChessMove, i32) {
    let res = game_state.game_result();

    match res {
        GameResult::WhiteWin => return (chess_move::NULL_MOVE, -CHECKMATE_VALUE - depth_left),
        GameResult::BlackWin => return (chess_move::NULL_MOVE, -CHECKMATE_VALUE - depth_left),
        GameResult::Draw => return (chess_move::NULL_MOVE, 0),
        GameResult::Undecided => (),
    }

    if depth_left == 0 {
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
        let (_, score) = negamax(game_state, depth_left - 1);
        game_state.undo_move();

        let score = -score;

        if score > best_score {
            best_score = score;
            best_move = m;
        }
    }

    return (best_move, best_score);
}

pub fn nega_alpha_beta(game_state: &mut GameState, max_depth: i32) -> (ChessMove, i32) {
    nega_alpha_beta_search(game_state, max_depth, -MAX_VALUE, MAX_VALUE)
}

fn nega_alpha_beta_search(game_state: &mut GameState, depth_left: i32, mut alpha: i32, beta: i32) -> (ChessMove, i32) {
    let res = game_state.game_result();

    match res {
        GameResult::WhiteWin => return (chess_move::NULL_MOVE, -CHECKMATE_VALUE - depth_left),
        GameResult::BlackWin => return (chess_move::NULL_MOVE, -CHECKMATE_VALUE - depth_left),
        GameResult::Draw => return (chess_move::NULL_MOVE, 0),
        GameResult::Undecided => (),
    }

    if depth_left == 0 {
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
        let (_, score) = nega_alpha_beta_search(game_state, depth_left - 1, -beta, -alpha);
        game_state.undo_move();

        let score = -score;

        if score > best_score {
            // println!("Improved eval: {}", score);

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