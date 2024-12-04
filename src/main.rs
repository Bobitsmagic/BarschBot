use std::{thread, time::Duration};

use barschbot::{board::{piece_type::ColoredPieceType, square}, evaluation::search_functions, game::game_state::GameState, gui::{engine_handle::{self, EngineHandle}, render_state::RenderState, vis_handle::VisHandle, visualizer::Visualizer}, moves::chess_move::{self, ChessMove}};
use rand::seq::SliceRandom;

fn main() {    
    let (vis_handle, engine_handle) = VisHandle::create_handles();
    
    let mut visualizer = Visualizer::new(engine_handle);

    //Start random move thread
    std::thread::spawn(move || {
        // random_moves(vis_handle);
        bot_battle(vis_handle);
    });

    visualizer.run();
}   

fn random_moves(engine_handle: VisHandle) {
    // let mut gs = GameState::start_position();
    let mut gs = GameState::from_fen("6k1/8/1R3K2/8/8/8/8/8 w - - 0 1");
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

fn bot_battle(engine_handle: VisHandle) {
    let mut gs = GameState::start_position();
    // let mut gs = GameState::from_fen("6k1/8/1R3K2/8/8/8/8/8 w - - 0 1");

    engine_handle.send_render_state(RenderState::render_move(
        gs.board_state.piece_board.clone(),
        chess_move::NULL_MOVE,
        false
    ));
    
    loop {
        // gs.board_state.piece_board.print();

        let moves = gs.gen_legal_moves();

        // for m in moves.iter() {
        //     m.print();
        // }

        loop {
            let uci = engine_handle.recive_move();

            if moves.contains(&uci) {
                gs.make_move(uci);
                engine_handle.send_render_state(RenderState::render_move(
                    gs.board_state.piece_board.clone(),
                    uci,
                    false
                ));

                break;
            }
        }

        // for d in 0..9 {
        //     println!("Depth: {}", d);
        //     let (m, eval) = search_functions::nega_alpha_beta(&mut gs, d);
        //     println!("Eval: {}", eval);
        // }


        let (m, eval, _) = search_functions::nega_alpha_beta(&mut gs, 8);

        gs.make_move(m);


        println!("Eval: {}", eval);

        engine_handle.send_render_state(RenderState::render_move(
            gs.board_state.piece_board.clone(),
            m,
            false
        ));
    }
}