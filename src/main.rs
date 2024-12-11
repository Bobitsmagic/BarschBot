use core::time;
use std::{thread, time::Duration};

use barschbot::{board::{piece_type::ColoredPieceType, square}, evaluation::search_functions, game::game_state::GameState, gui::{engine_handle::{self, EngineHandle}, render_state::RenderState, vis_handle::VisHandle, visualizer::Visualizer}, match_handling, moves::chess_move::{self, ChessMove}};
use piston_window::color::BLACK;
use rand::seq::SliceRandom;

fn main() {    
    let (vis_handle, engine_handle) = VisHandle::create_handles();
    
    let mut visualizer = Visualizer::new(engine_handle);

    //Start random move thread
    std::thread::spawn(move || {
        // random_moves(vis_handle);
        human_against_bot(vis_handle);
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


//Error at r6k/1bpp1pp1/2q1r2p/p3PQ2/4BP2/P1B3R1/1PP3PP/2KR4 b - - 0 23
fn human_against_bot(engine_handle: VisHandle) {
    const PLAY_BLACK: bool = false;

    
    // let mut rng = rand::thread_rng();
    // let fen_list = match_handling::file_loader::load_test_fens();
    
    // let mut gs = fen_list.choose(&mut rng).unwrap().clone();
    
    let mut gs = GameState::start_position();
    // let mut gs = GameState::from_fen("1k5r/p1p2ppp/1pn1p3/8/3P4/Q1PqB2P/5PPK/8 w - - 0 1");
    
    const START_TIME : u128 = 1000 * 60 * 5;
    let mut white_time_left = START_TIME;
    let mut black_time_left = START_TIME;

    engine_handle.send_render_state(RenderState::render_move_timed(
        gs.board_state.piece_board.clone(),
        chess_move::NULL_MOVE,
        PLAY_BLACK,
        white_time_left,
        black_time_left
    ));

    if PLAY_BLACK {
        let (m, time_used) = get_bot_move(&mut gs, black_time_left);
        gs.make_move(m);

        white_time_left -= time_used.min(white_time_left);

        engine_handle.send_render_state(RenderState::render_move_timed(
            gs.board_state.piece_board.clone(),
            m,
            PLAY_BLACK,
            white_time_left,
            black_time_left
        ));
    }

    loop { 
        let (m, time_used) = get_human_move(&mut gs, &engine_handle);
        gs.make_move(m);

        if PLAY_BLACK {
            black_time_left -= time_used.min(black_time_left);
        } else {
            white_time_left -= time_used.min(white_time_left);
        }

        engine_handle.send_render_state(RenderState::render_move_timed(
            gs.board_state.piece_board.clone(),
            m,
            PLAY_BLACK,
            white_time_left,
            black_time_left
        ));

        let (m, time_used) = get_bot_move(&mut gs, black_time_left);
        gs.make_move(m);

        if PLAY_BLACK {
            white_time_left -= time_used.min(white_time_left);
        } else {
            black_time_left -= time_used.min(black_time_left);
        }

        engine_handle.send_render_state(RenderState::render_move_timed(
            gs.board_state.piece_board.clone(),
            m,
            PLAY_BLACK,
            white_time_left,
            black_time_left
        ));
    }

    fn get_bot_move(gs: &mut GameState, time_left: u128) -> (ChessMove, u128) {
        let start_time = std::time::Instant::now();
        let (m, _, _) = search_functions::timed_search(gs, time_left);
        let time_used = start_time.elapsed().as_millis();
        (m, time_used)
    }

    fn get_human_move(gs: &mut GameState, engine_handle: &VisHandle) -> (ChessMove, u128) {
        let start_time = std::time::Instant::now();
        let moves = gs.gen_legal_moves();
        loop {
            let uci = engine_handle.recive_move();

            if moves.contains(&uci) {
                return (uci, start_time.elapsed().as_millis());
            }
        }
    }
}