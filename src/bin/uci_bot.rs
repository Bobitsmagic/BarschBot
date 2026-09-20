use std::io;

use barschbot::{
    board::player_color::PlayerColor::{Black, White}, evaluation::{
        barschbot::Barschbot,
        hans_eval::{Attributes, EvaluationSettings, STANDARD_EVAL},
        settings::{self, Settings},
    }, game::game_state::GameState, moves::uci_move::UciMove, tablebase::opening_book::{self, OpeningBook},
};

const VERSION: &str = "0.0.2";

fn main() {
    let mut bot = Barschbot::named(
        Settings {
            time_percentage: 0.015,
            quiessence_depth: 5,
            check_extensions: 0,
            null_move_pruning: 0,
            evaluation_mode: settings::EvaluationMode::HansEvaluation(EvaluationSettings {
                use_new_feature: false,
                attr_weights: Attributes {
                    mobility: [0, 50, 40, 10, 5, 0],
                    ..STANDARD_EVAL
                },
                // attr_weights: Attributes {passed_pawn: 100, ..hans_eval::STANDARD_EVAL},
            }),
        },
        String::from("Hans"),
    );

    let mut book = OpeningBook::load_from_file("data/book.txt");

    let mut gs = GameState::start_position();

    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let mut split = buffer.trim().split(" ");

        match split.next().unwrap() {
            "uci" => {
                println!("id name Barschbot {}", VERSION);
                println!("id author Bobitsmagic");
                println!("uciok");
            }
            "isready" => println!("readyok"),
            "ucinewgame" => {}
            "position" => {
                gs = match split.next().expect(&format!("Error at {}", buffer)) {
                    "startpos" => GameState::start_position(),
                    "fen" => {
                        GameState::from_fen(split.next().expect(&format!("Error at {}", buffer)))
                    }
                    _ => panic!("Error during position command: {}", buffer),
                };

                gs.visited_pos.clear();

                if let Some(next_key) = split.next() {
                    assert_eq!(next_key, "moves");

                    while let Some(m_uci) = split.next() {
                        let um = UciMove::from_str(m_uci);
                        for cm in gs.gen_legal_moves() {
                            if cm.uci_move() == um {
                                gs.make_move(cm);
                                break;
                            }
                        }
                    }
                }
            }
            "go" => {
                let bm = if let Some(m) = book.get_move(&gs) {
                    m
                }
                else {

                    //go movetime 10000
                    //go wtime 590839 btime 533180 winc 0 binc 0
    
                    let move_time = match split
                        .next()
                        .expect(&format!("Could not resolve go parameter at {}", buffer))
                    {
                        "movetime" => 1000_u128 * 200,
                        "wtime" => {
                            let wtime = split.next().unwrap().parse::<u128>().unwrap();
                            split.next();
                            let btime = split.next().unwrap().parse::<u128>().unwrap();
    
                            split.next();
                            let winc = split.next().unwrap().parse::<u128>().unwrap();
                            split.next();
                            let binc = split.next().unwrap().parse::<u128>().unwrap();
    
                            let (flat_time, inc) = match gs.active_color() {
                                White => (wtime, winc),
                                Black => (btime, binc),
                            };
    
                            flat_time + inc * 20
                        }
                        _ => panic!("Unexpected time format during go command: {}", buffer),
                    } * 1000;
    
                    let bm = bot.search_time(&mut gs, move_time);

                    bm
                };

                println!("bestmove {}", bm.uci_move().to_string());
            }
            "quit" => {
                return;
            }

            x => panic!("Unknown command type: [{}]", x),
        }
    }
}
