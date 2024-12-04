use crate::{board::player_color::PlayerColor, game::{board_state::BoardState, game_state::GameState}, moves::{chess_move::{self, ChessMove}, move_gen}};

use super::attributes::{self, Attributes};

pub fn negamax(game_state: &mut GameState, depth: i32) -> (ChessMove, i32) {
    if depth == 0 {
        let factor = match game_state.active_color() {
            PlayerColor::White => 1,
            PlayerColor::Black => -1,
        };
        return (chess_move::NULL_MOVE, factor * Attributes::from_board_state(&game_state.board_state).multiply(&attributes::STANDARD_EVAL));
    }

    let mut best_move = chess_move::NULL_MOVE;
    let mut best_score = i32::MIN;

    for m in move_gen::gen_legal_moves(&game_state.board_state, &game_state.get_flags()) {
        
        game_state.make_move(m);
        let (_, score) = negamax(game_state, depth - 1);
        game_state.undo_move();

        let score = -score;

        if score > best_score {
            best_score = score;
            best_move = m;
        }
    }

    return (best_move, best_score);
}