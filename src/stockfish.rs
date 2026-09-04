use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    process::{Child, Command, Stdio},
};

use crate::{game::game_state::GameState, moves::chess_move::ChessMove};

pub fn get_stock_fish_process() -> Child {
    return Command::new("stockfish\\stockfish-windows-x86-64-avx2.exe")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
}

pub fn get_stock_fish_move(game: &mut GameState, cmd: &mut Child) -> (ChessMove, i32) {
    const DEPTH: u8 = 20;

    let stdin = cmd.stdin.as_mut().unwrap();
    let mut stdin_writer = BufWriter::new(stdin);
    {
        stdin_writer
            .write_all(format!("position fen {}\n", game.to_fen()).as_bytes())
            .unwrap();
        stdin_writer.flush().unwrap();
        stdin_writer
            .write_all(format!("go depth {}\n", DEPTH).as_bytes())
            .unwrap();
        stdin_writer.flush().unwrap();
    }

    let stdout = cmd.stdout.as_mut().unwrap();

    let mut stdout_reader = BufReader::new(stdout);

    let mut prev = String::new();
    loop {
        let mut s = String::new();
        stdout_reader.read_line(&mut s).expect("error");

        // println!("{}", s);
        if s.starts_with("bestmove") {
            let score: i32 = prev.split(" ").skip(9).next().unwrap().parse().unwrap();

            // println!("Score: {}", score);

            // s.split(" ").skip(1).next().unwrap();

            let parts = s.split(" ").collect::<Vec<_>>();
            let length = parts[1].len() - 2;

            let bms = if parts.len() == 2 {
                &parts[1][..length]
            } else {
                parts[1]
            };

            //println!("SF: [{}]", bms);
            let list = game.gen_legal_moves();

            for m in list {
                if m.uci_move().to_string() == bms {
                    return (m, score);
                }
            }

            break;
        }

        prev = s;
    }

    panic!("Stockfish made an illegal move?");
}
