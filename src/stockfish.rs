use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    process::{Child, Command, Stdio},
};

use crate::{
    game::game_state::GameState,
    moves::{
        chess_move::ChessMove,
        uci_move::{self, UciMove},
    },
};

pub struct StockFishBot {
    pub max_time: Option<u128>,
    pub max_depth: Option<u32>,
    pub process: Child,
}

impl Default for StockFishBot {
    fn default() -> Self {
        StockFishBot {
            max_time: None,
            max_depth: None,
            process: get_stock_fish_process(),
        }
    }
}

impl StockFishBot {
    pub fn get_uci_move(&mut self, fen: String) -> (UciMove, i32) {
        let stdin = self.process.stdin.as_mut().unwrap();
        let mut stdin_writer = BufWriter::new(stdin);

        let mut go_string = "go".to_owned();
        if let Some(md) = self.max_depth {
            go_string += &format!(" depth {}", md)
        }
        if let Some(mt) = self.max_time {
            go_string += &format!(" movetime {}", mt)
        }

        stdin_writer
            .write_all(format!("position fen {}\n", fen).as_bytes())
            .unwrap();
        stdin_writer.flush().unwrap();
        stdin_writer
            .write_all(format!("{go_string}\n").as_bytes())
            .unwrap();
        stdin_writer.flush().unwrap();

        let stdout = self.process.stdout.as_mut().unwrap();

        let mut stdout_reader = BufReader::new(stdout);

        let mut prev = String::new();
        loop {
            let mut s = String::new();
            stdout_reader.read_line(&mut s).expect("error");

            if s.starts_with("bestmove") {
                let score: i32 = if let Some(score_string) = prev.split(" ").skip(9).next() {
                    score_string.parse().unwrap()
                } else {
                    return (uci_move::NULL_MOVE, 0);
                };

                let parts = s.split(" ").collect::<Vec<_>>();
                let length = parts[1].len() - 2;

                let bms = if parts.len() == 2 {
                    &parts[1][..length]
                } else {
                    parts[1]
                };

                let uci = UciMove::from_str(bms);

                return (uci, score);
            }

            prev = s;
        }
    }

    pub fn get_best_move(&mut self, game: &mut GameState) -> (ChessMove, i32) {
        let (uci, score) = self.get_uci_move(game.to_fen());

        let list = game.gen_legal_moves();

        for m in list {
            if m.uci_move() == uci {
                return (m, score);
            }
        }

        panic!("Stockfish returned illegal move");
    }
}

fn get_stock_fish_process() -> Child {
    return Command::new("stockfish\\stockfish.exe")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
}
