use std::{f64::consts::SQRT_2, thread, time::Duration};

use barschbot::{
    evaluation::{
        barschbot::Barschbot,
        hans_eval::{self, EvaluationSettings},
        settings::{self, Settings},
        wiesel_eval::WieselSettings,
    },
    game::game_state::GameState,
    gui::{render_state::RenderState, vis_handle::VisHandle, visualizer::Visualizer},
    match_handling::match_handler,
    moves::chess_move::{self, ChessMove},
};
use rand::seq::SliceRandom;
//Wins Old version: 358, Wins New version: 499, Draws: 143

fn main() {
    // start_human_against_bot();

    // rayon::ThreadPoolBuilder::new().num_threads(12).build_global().unwrap();
    let bot_a = Barschbot::named(
        Settings {
            time_percentage: 0.015,
            quiessence_depth: 5,
            check_extensions: 2,
            evaluation_mode: settings::EvaluationMode::HansEvaluation(EvaluationSettings {
                use_new_feature: false,
                attr_weights: hans_eval::STANDARD_EVAL,
            }),
        },
        String::from("New Hans"),
    );

    // let bot_b = Barschbot::named(
    //     Settings {
    //         time_percentage: 0.02,
    //         quiessence_depth: 5,
    //         check_extensions: 0,
    //         evaluation_mode: settings::EvaluationMode::HansEvaluation(EvaluationSettings {
    //             use_new_feature: false,
    //             attr_weights: hans_eval::STANDARD_EVAL,
    //         }),
    //     },
    //     String::from("Old hans"),
    // );

    let bot_b = Barschbot::named(
        Settings {
            time_percentage: 0.02,
            quiessence_depth: 5,
            check_extensions: 0,
            evaluation_mode: settings::EvaluationMode::WieselEvaluation(WieselSettings {
                pawn_value: 1000,
                version: 3,
                piece_weight: [1000, 3000, 3000, 5000, 9000],
            }),
        },
        String::from("Wiesel"),
    );

    // play_all_fens_vis(bot_b.clone(), bot_a.clone());
    play_all_fens_par(bot_a, bot_b);
}

fn play_all_fens_vis(mut bot_a: Barschbot, mut bot_b: Barschbot) {
    let (vis_handle, engine_handle) = VisHandle::create_handles();

    let mut visualizer = Visualizer::new(engine_handle);

    //Start random move thread
    std::thread::spawn(move || {
        // random_moves(vis_handle);
        let (a_wins, b_wins, draws) =
            match_handler::show_all_fens(&mut bot_a, &mut bot_b, 1000 * 60, vis_handle);
        println!(
            "Finished: {} wins: {}, {} wins: {}, Draws: {}",
            bot_a.name, a_wins, bot_b.name, b_wins, draws
        );
    });

    visualizer.run();
}

fn probability_of_superiority(a_wins: i32, b_wins: i32, draws: i32) -> f64 {
    use errorfunctions::RealErrorFunctions;

    let sum = a_wins + b_wins + draws;
    let mean = sum as f64 / 2.0;
    let std = mean * 0.5;
    let b_score = b_wins as f64 + draws as f64 * 0.5;

    let a_better_than_b = 1.0 - 0.5 * (1.0 + ((b_score - mean) / (std * SQRT_2)).erf());

    return a_better_than_b;
}

fn play_all_fens_par(mut bot_a: Barschbot, mut bot_b: Barschbot) {
    let stats = match_handler::play_all_fens(&mut bot_a, &mut bot_b, 1000 * 1000 * 10);
    
    stats.print_wins(&bot_a.name, &bot_b.name);
    println!("Probability of {} being superior to {}: {:.3}", bot_a.name, bot_b.name, probability_of_superiority(stats.a_wins(), stats.b_wins(), stats.draws()));

    stats.print_stats(&bot_a.name, &bot_b.name);
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

        let rs = RenderState::render_move(gs.board_state.piece_board.clone(), *random_move, false);

        thread::sleep(Duration::from_millis(100));
        engine_handle.send_render_state(rs);
    }
}

//Error at r6k/1bpp1pp1/2q1r2p/p3PQ2/4BP2/P1B3R1/1PP3PP/2KR4 b - - 0 23
fn start_human_against_bot() {
    let (vis_handle, engine_handle) = VisHandle::create_handles();

    let mut visualizer = Visualizer::new(engine_handle);

    //Start random move thread
    std::thread::spawn(move || {
        human_against_bot(vis_handle);
    });

    visualizer.run();
}
fn human_against_bot(engine_handle: VisHandle) {
    const PLAY_BLACK: bool = false;

    // let mut rng = rand::thread_rng();
    // let fen_list = match_handling::file_loader::load_test_fens();

    // let mut gs = fen_list.choose(&mut rng).unwrap().clone();

    // let mut gs = GameState::start_position();
    let mut gs = GameState::from_fen("8/8/2r1k3/8/3K4/8/8/8 w - - 0 1"); //Rook endgame
                                                                         // let mut gs = GameState::from_fen("8/8/2bbk3/8/3K4/8/8/8 w - - 0 1"); //Bishop endgame

    const START_TIME: u128 = 1000 * 60 * 1;
    let mut white_time_left = START_TIME;
    let mut black_time_left = START_TIME;

    let mut bot = Barschbot::named(
        Settings {
            time_percentage: 0.02,
            quiessence_depth: 5,
            check_extensions: 0,
            evaluation_mode: settings::EvaluationMode::HansEvaluation(EvaluationSettings {
                use_new_feature: false,
                attr_weights: hans_eval::STANDARD_EVAL,
            }),
        },
        String::from("Waldwiesel destroyer"),
    );

    engine_handle.send_render_state(RenderState::render_move_timed(
        gs.board_state.piece_board.clone(),
        chess_move::NULL_MOVE,
        PLAY_BLACK,
        white_time_left,
        black_time_left,
    ));

    if PLAY_BLACK {
        let (m, time_used) = get_bot_move(&mut gs, black_time_left, &mut bot);
        gs.make_move(m);

        white_time_left -= time_used.min(white_time_left);

        engine_handle.send_render_state(RenderState::render_move_timed(
            gs.board_state.piece_board.clone(),
            m,
            PLAY_BLACK,
            white_time_left,
            black_time_left,
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
            black_time_left,
        ));

        let (m, time_used) = get_bot_move(
            &mut gs,
            if PLAY_BLACK {
                white_time_left
            } else {
                black_time_left
            },
            &mut bot,
        );
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
            black_time_left,
        ));
    }

    fn get_bot_move(gs: &mut GameState, time_left: u128, bot: &mut Barschbot) -> (ChessMove, u128) {
        let start_time = std::time::Instant::now();
        // let (m, _, _) = search_functions::timed_search(gs, time_left);
        let m = bot.search(gs, time_left);
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
