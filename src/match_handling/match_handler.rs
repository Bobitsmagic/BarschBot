use std::{default, sync::{Arc, Mutex}};

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    board::player_color::PlayerColor::{self, Black, White}, evaluation::barschbot::Barschbot, game::{game_result::{DrawType::{FiftyMoveRule, InsufficientMaterial, Repetition, StaleMate}, GameResult::{self, Win}, WinType::{Checkmate, TimeOut}}, game_state::GameState}, gui::{render_state::RenderState, vis_handle::VisHandle}, moves::chess_move,
};

#[derive(Clone)]
pub struct MatchStats {
    pub a_cm: i32,
    pub a_time: i32,
    pub b_cm: i32,
    pub b_time: i32,
    pub insuff: i32,
    pub repe: i32,
    pub stalemate: i32,
    pub fifty: i32,
}

impl Default for MatchStats {
    fn default() -> Self {
        MatchStats { a_cm: 0, a_time: 0, b_cm: 0, b_time: 0, insuff: 0, repe: 0, stalemate: 0, fifty: 0 }
    }
}

impl MatchStats {
    pub fn print_wins(&self, name_a: &str, name_b: &str) {
        println!("Wins {}: {} Wins {}: {} Draws: {}", name_a, self.a_wins(), name_b, self.b_wins(), self.draws())
    }

    pub fn a_wins(&self) -> i32 {
        self.a_cm + self.a_time
    }
    pub fn b_wins(&self) -> i32 {
        self.b_cm + self.b_time
    }
    pub fn draws(&self) -> i32 {
        self.insuff + self.repe + self.stalemate + self.fifty
    }

    pub fn handle_game_result(&mut self, res: GameResult, a_color: PlayerColor) {
        match res {

            Win(wc, Checkmate) => {
                if wc == a_color {
                    self.a_cm += 1;
                }
                else {
                    self.b_cm += 1;
                }
            },
            Win(wc, TimeOut) => {
                if wc == a_color {
                    self.a_time += 1;
                }
                else {
                    self.b_time += 1;
                }
            }

            GameResult::Draw(InsufficientMaterial) => self.insuff += 1,
            GameResult::Draw(Repetition) => self.repe += 1,
            GameResult::Draw(StaleMate) => self.stalemate += 1,
            GameResult::Draw(FiftyMoveRule) => self.fifty += 1,

            GameResult::Undecided => panic!("Kek"),
        }
    }

    pub fn print_stats(&self, name_a: &str, name_b: &str) {
        println!("Checkmate {name_a}: {} {name_b}: {}", self.a_cm, self.b_cm);
        println!("Time out {name_a}: {} {name_b}: {}", self.a_time, self.b_time);
        
        println!("Insufficient material {}", self.insuff);
        println!("Repetition {}", self.repe);
        println!("Stalemate {}", self.stalemate);
        println!("50 move rule {}", self.fifty);
    }
}


pub fn play_timed_game(
    game_state: &mut GameState,
    bot_a: &mut Barschbot,
    bot_b: &mut Barschbot,
    start_time_mu_s: u128,
) -> GameResult {
    let mut time_left_a = start_time_mu_s;
    let mut time_left_b = start_time_mu_s;

    loop {
        let mut res = game_state.game_result();

        //Bot a
        if res != GameResult::Undecided {
            return res;
        }

        let mut start_time = std::time::Instant::now();
        let m = bot_a.search(game_state, time_left_a);
        let time_used = start_time.elapsed().as_micros();

        if time_used > time_left_a {
            return GameResult::Win(!game_state.active_color(), TimeOut)
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
        let time_used = start_time.elapsed().as_micros();

        if time_used > time_left_b {
            return GameResult::Win(!game_state.active_color(), TimeOut)
        }
        time_left_b -= time_used;
        game_state.make_move(m);
    }
}

pub fn show_timed_game(
    gs: &mut GameState,
    bot_a: &mut Barschbot,
    bot_b: &mut Barschbot,
    start_time_ms: u128,
    engine_handle: &VisHandle,
) -> GameResult {
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
            return GameResult::Win(!gs.active_color(), TimeOut)
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
            return GameResult::Win(!gs.active_color(), TimeOut)
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

pub fn play_all_fens(
    bot_a: &mut Barschbot,
    bot_b: &mut Barschbot,
    start_time_mu_s: u128,
) ->  MatchStats {
    let fens = crate::match_handling::file_loader::load_test_fens();

    let win_counter = Arc::new(Mutex::new(MatchStats::default()));

    let mut list = Vec::new();
    for f in fens {
        list.push((f, win_counter.clone(), bot_a.clone(), bot_b.clone()));
    }

    list.par_iter_mut()
        .for_each(move |(fen, win_counter, bot_a, bot_b)| {
            let mut game_state = GameState::from_fen(&fen.to_fen());

            let mut bot_a = bot_a.clone();
            let mut bot_b = bot_b.clone();

            let start_color = game_state.active_color();
            let res = play_timed_game(&mut game_state, &mut bot_a, &mut bot_b, start_time_mu_s);
            win_counter.lock().unwrap().handle_game_result(res, start_color);

            game_state = GameState::from_fen(&fen.to_fen());
            let res = play_timed_game(&mut game_state, &mut bot_b, &mut bot_a, start_time_mu_s);
            let mut lock = win_counter.lock().unwrap();
            lock.handle_game_result(res, !start_color);
            lock.print_wins(&bot_a.name, &bot_b.name);
        });

    let stats = win_counter.lock().unwrap();
    
    return stats.clone();
}

pub fn show_all_fens(
    bot_a: &mut Barschbot,
    bot_b: &mut Barschbot,
    start_time_ms: u128,
    engine_handle: VisHandle,
) -> (i32, i32, i32) {
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
            GameResult::Win(win_color, _) => {
                if win_color == start_color {
                    b_wins += 1;
                }
                else {
                    a_wins += 1;
                }
            },
            
            GameResult::Draw(_) => draws += 1,
            GameResult::Undecided => panic!("Finished on undecided game")
        }

        game_state = GameState::from_fen(&fen.to_fen());
        let res = show_timed_game(&mut game_state, bot_b, bot_a, start_time_ms, &engine_handle);
        game_state.board_state.piece_board.print();
        match res {
            GameResult::Win(win_color, _) => {
                if win_color == start_color {
                    b_wins += 1;
                }
                else {
                    a_wins += 1;
                }
            },
            
            GameResult::Draw(_) => draws += 1,
            GameResult::Undecided => panic!("Finished on undecided game")
        }

        println!(
            "{} wins: {}, {} wins: {}, Draws: {}",
            bot_a.name, a_wins, bot_b.name, b_wins, draws
        );
    }

    (a_wins, b_wins, draws)
}
