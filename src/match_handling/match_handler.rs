use std::sync::{Arc, Mutex};

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::{board::player_color::PlayerColor, evaluation::barschbot::Barschbot, game::{game_result::GameResult, game_state::{self, GameState}}, gui::{render_state::RenderState, vis_handle::VisHandle}, moves::chess_move};

pub fn play_timed_game(game_state: &mut GameState, bot_a: &mut Barschbot, bot_b: &mut Barschbot, start_time_ms: u128) -> GameResult {
    let mut time_left_a = start_time_ms;
    let mut time_left_b = start_time_ms;

    loop {
        let mut res = game_state.game_result();

        //Bot a
        if res != GameResult::Undecided {
            return res;
        }

        let mut start_time = std::time::Instant::now();
        let m = bot_a.search(game_state, time_left_a);
        let time_used = start_time.elapsed().as_millis();

        if time_used > time_left_a {
            match game_state.active_color() {
                PlayerColor::White => return GameResult::BlackWin,
                PlayerColor::Black => return GameResult::WhiteWin,
            }
        }
        time_left_a -= time_used;   
        game_state.make_move(m);

        //Bot b
        res = game_state.game_result();
        if res != GameResult::Undecided {
            return res;
        }

        start_time = std::time::Instant::now();
        let m = bot_b.search(game_state, time_left_b);
        let time_used = start_time.elapsed().as_millis();

        if time_used > time_left_b {
            match game_state.active_color() {
                PlayerColor::White => return GameResult::BlackWin,
                PlayerColor::Black => return GameResult::WhiteWin,
            }
        }
        time_left_b -= time_used;   
        game_state.make_move(m);
    }
}

pub fn show_timed_game(gs: &mut GameState, bot_a: &mut Barschbot, bot_b: &mut Barschbot, start_time_ms: u128, engine_handle: &VisHandle) -> GameResult {
    let mut time_left_a = start_time_ms;
    let mut time_left_b = start_time_ms;

    let a_color = gs.active_color();
    engine_handle.send_render_state(RenderState::render_move_named(
        gs.board_state.piece_board.clone(),
        chess_move::NULL_MOVE,
        false,
        match a_color {
            PlayerColor::White => time_left_a,
            PlayerColor::Black => time_left_b,
        },
        match a_color {
            PlayerColor::White => time_left_b,
            PlayerColor::Black => time_left_a,
        },
        match a_color {
            PlayerColor::White => bot_a.name.clone(),
            PlayerColor::Black => bot_b.name.clone(),
        },
        match a_color {
            PlayerColor::White => bot_b.name.clone(),
            PlayerColor::Black => bot_a.name.clone(),
        },
    ));

    loop {
        let mut res = gs.game_result();

        //Bot a
        if res != GameResult::Undecided {
            println!("Game result: {:?}", res);
            return res;
        }

        let mut start_time = std::time::Instant::now();
        let m = bot_a.search(gs, time_left_a);
        let time_used = start_time.elapsed().as_millis();

        if time_used > time_left_a {
            match gs.active_color() {
                PlayerColor::White => return GameResult::BlackWin,
                PlayerColor::Black => return GameResult::WhiteWin,
            }
        }
        time_left_a -= time_used;   
        gs.make_move(m);

        engine_handle.send_render_state(RenderState::render_move_named(
            gs.board_state.piece_board.clone(),
            m,
            false,
            match a_color {
                PlayerColor::White => time_left_a,
                PlayerColor::Black => time_left_b,
            },
            match a_color {
                PlayerColor::White => time_left_b,
                PlayerColor::Black => time_left_a,
            },
            match a_color {
                PlayerColor::White => bot_a.name.clone(),
                PlayerColor::Black => bot_b.name.clone(),
            },
            match a_color {
                PlayerColor::White => bot_b.name.clone(),
                PlayerColor::Black => bot_a.name.clone(),
            },
        ));

        //Bot b
        res = gs.game_result();
        if res != GameResult::Undecided {
            println!("Game result: {:?}", res);
            return res;
        }

        start_time = std::time::Instant::now();
        let m = bot_b.search(gs, time_left_b);
        let time_used = start_time.elapsed().as_millis();

        if time_used > time_left_b {
            match gs.active_color() {
                PlayerColor::White => return GameResult::BlackWin,
                PlayerColor::Black => return GameResult::WhiteWin,
            }
        }

        time_left_b -= time_used;   
        gs.make_move(m);
        
        engine_handle.send_render_state(RenderState::render_move_named(
            gs.board_state.piece_board.clone(),
            m,
            false,
            match a_color {
                PlayerColor::White => time_left_a,
                PlayerColor::Black => time_left_b,
            },
            match a_color {
                PlayerColor::White => time_left_b,
                PlayerColor::Black => time_left_a,
            },
            match a_color {
                PlayerColor::White => bot_a.name.clone(),
                PlayerColor::Black => bot_b.name.clone(),
            },
            match a_color {
                PlayerColor::White => bot_b.name.clone(),
                PlayerColor::Black => bot_a.name.clone(),
            },
        ));

    }
}

pub fn play_all_fens(bot_a: &mut Barschbot, bot_b: &mut Barschbot, start_time_ms: u128) -> (i32, i32, i32) {
    let fens = crate::match_handling::file_loader::load_test_fens();


    let win_counter = Arc::new(Mutex::new((0, 0, 0)));
      
    let mut list = Vec::new();
    for f in fens {
        list.push((f, win_counter.clone(), bot_a.clone(), bot_b.clone()));
    }
    
    list.par_iter_mut().for_each(move |(fen, win_counter, bot_a, bot_b)| {
        let mut game_state = GameState::from_fen(&fen.to_fen());
        
        let mut bot_a = bot_a.clone();
        let mut bot_b = bot_b.clone();
        let mut a_wins = 0;
        let mut b_wins = 0;
        let mut draws = 0;

        let start_color = game_state.active_color();
        let res = play_timed_game(&mut game_state, &mut bot_a, &mut bot_b, start_time_ms);

        match res {
            GameResult::WhiteWin => if start_color == PlayerColor::White { a_wins += 1 } else { b_wins += 1 },
            GameResult::BlackWin => if start_color == PlayerColor::Black { a_wins += 1 } else { b_wins += 1 },
            GameResult::Draw => draws += 1,
            _ => (),
        }
        // game_state.board_state.piece_board.print();

        game_state = GameState::from_fen(&fen.to_fen());
        let res = play_timed_game(&mut game_state, &mut bot_b, &mut bot_a, start_time_ms);

        match res {
            GameResult::WhiteWin => if start_color == PlayerColor::White { b_wins += 1 } else { a_wins += 1 },
            GameResult::BlackWin => if start_color == PlayerColor::Black { b_wins += 1 } else { a_wins += 1 },
            GameResult::Draw => draws += 1,
            _ => (),
        }

        let mut tuple = win_counter.lock().unwrap();

        tuple.0 += a_wins;
        tuple.1 += b_wins;
        tuple.2 += draws;

        println!("Wins {}: {}, Wins {}: {}, Draws: {}", bot_a.name, tuple.0, bot_b.name, tuple.1, tuple.2);
    });
        

    let tuple = win_counter.lock().unwrap();
    let a_wins = tuple.0;
    let b_wins = tuple.1;
    let draws = tuple.2;

    (a_wins, b_wins, draws)
}

pub fn show_all_fens(bot_a: &mut Barschbot, bot_b: &mut Barschbot, start_time_ms: u128, engine_handle: VisHandle) -> (i32, i32, i32) {
    let fens = crate::match_handling::file_loader::load_test_fens();
    let mut a_wins = 0;
    let mut b_wins = 0;
    let mut draws = 0;

    for fen in fens.iter().into_iter().skip(0) {
        let mut game_state = GameState::from_fen(&fen.to_fen());
        
        let start_color = game_state.active_color();
        let res = show_timed_game(&mut game_state, bot_a, bot_b, start_time_ms, &engine_handle);
        game_state.board_state.piece_board.print();
        match res {
            GameResult::WhiteWin => if start_color == PlayerColor::White { a_wins += 1 } else { b_wins += 1 },
            GameResult::BlackWin => if start_color == PlayerColor::Black { a_wins += 1 } else { b_wins += 1 },
            GameResult::Draw => draws += 1,
            _ => (),
        }

        game_state = GameState::from_fen(&fen.to_fen());
        let res = show_timed_game(&mut game_state, bot_b, bot_a, start_time_ms, &engine_handle);
        game_state.board_state.piece_board.print();
        match res {
            GameResult::WhiteWin => if start_color == PlayerColor::White { b_wins += 1 } else { a_wins += 1 },
            GameResult::BlackWin => if start_color == PlayerColor::Black { b_wins += 1 } else { a_wins += 1 },
            GameResult::Draw => draws += 1,
            _ => (),
        }

        println!("Wins: A: {}, B: {}, Draws: {}", a_wins, b_wins, draws);
    }

    (a_wins, b_wins, draws)
}