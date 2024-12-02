use std::{thread, time::Duration};

use barschbot::{board::{piece_type::ColoredPieceType, square}, game::game_state::GameState, gui::{engine_handle::{self, EngineHandle}, render_state::RenderState, vis_handle::VisHandle, visualizer::Visualizer}, moves::chess_move::ChessMove};
use rand::seq::SliceRandom;

fn main() {    
    let (vis_handle, engine_handle) = VisHandle::create_handles();
    
    let mut visualizer = Visualizer::new(engine_handle);

    //Start random move thread
    std::thread::spawn(move || {
        random_moves(vis_handle);
    });

    visualizer.run();
}   

fn random_moves(engine_handle: VisHandle) {
    let mut gs = GameState::start_position();
    // let mut gs = GameState::from_fen("8/6PP/8/8/8/8/K1k5/8 w - - 0 1");
    let mut rng = rand::thread_rng();
    
    loop {
        let moves = gs.gen_legal_moves();
        if moves.len() == 0 {
            break;
        }
        let random_move = moves.choose(&mut rng).unwrap();
        gs.make_move(*random_move);

        let rs = RenderState::render_move(
            gs.board_state.piece_board.clone(),
            *random_move,
            false
        );

        thread::sleep(Duration::from_millis(100));
        engine_handle.send_render_state(rs);
    }
}