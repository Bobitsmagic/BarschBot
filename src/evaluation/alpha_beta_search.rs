use crate::{
    board::player_color::PlayerColor,
    evaluation::{
        search_functions::{better_move_sorter, CHECKMATE_VALUE, MAX_VALUE},
        settings::Settings,
    },
    game::{game_result::GameResult, game_state::GameState},
    moves::chess_move::{self, ChessMove},
};

pub fn go_depth(game_state: &mut GameState, depth: i32, settings: &Settings) -> (ChessMove, i32) {
    let mut all_moves = game_state.gen_legal_moves();

    let mut best_move = chess_move::NULL_MOVE;
    let mut best_score = -MAX_VALUE;

    better_move_sorter(
        &mut all_moves,
        &game_state.board_state,
        chess_move::NULL_MOVE,
    );

    for m in all_moves {
        game_state.make_move(m);

        // m.print();
        let score = -alpha_beta_nega(game_state, 1, depth - 1, -MAX_VALUE, -best_score, settings);
        // println!("Score: {}", score);
        if score > best_score {
            best_score = score;
            best_move = m;
        }

        game_state.undo_move();
    }

    return (best_move, best_score);
}

fn alpha_beta_nega(
    game_state: &mut GameState,
    depth: i32,
    depth_left: i32,
    mut lower_bound: i32,
    upper_bound: i32,
    settings: &Settings,
) -> i32 {
    let res = game_state.game_result();
    match res {
        GameResult::Win(_, _) => return -CHECKMATE_VALUE + depth,
        GameResult::Draw(_) => return 0,
        GameResult::Undecided => (),
    }

    if depth_left <= 0 {
        let factor = match game_state.active_color() {
            PlayerColor::White => 1,
            PlayerColor::Black => -1,
        };

        return settings.evaluate(game_state, lower_bound, upper_bound) * factor;
    }

    //Beta cutoff due to zero window
    if lower_bound >= upper_bound {
        return upper_bound;
    }

    let (mut lm, _) = game_state.gen_legal_moves_check();

    better_move_sorter(&mut lm, &game_state.board_state, chess_move::NULL_MOVE);

    for m in lm {
        game_state.make_move(m);

        let score = -alpha_beta_nega(
            game_state,
            depth + 1,
            depth_left - 1,
            -upper_bound,
            -lower_bound,
            settings,
        );

        game_state.undo_move();

        if score > lower_bound {
            lower_bound = score;
        }

        if lower_bound >= upper_bound {
            return lower_bound;
        }
    }

    return lower_bound;
}
