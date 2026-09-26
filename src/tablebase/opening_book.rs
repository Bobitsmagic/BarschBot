use core::panic;
use std::{collections::HashMap, fs, str::Lines};

use rand::{
    rngs::{self, StdRng},
    Rng, SeedableRng,
};
use rayon::{
    iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator},
    slice::ParallelSlice,
};

use crate::{
    game::game_state::{self, GameState},
    moves::chess_move::ChessMove,
    stockfish::StockFishBot,
};

pub struct OpeningBook {
    rng: StdRng,
    pub move_map: HashMap<u64, (Vec<ChessMove>, Vec<i32>)>,
}

impl Default for OpeningBook {
    fn default() -> Self {
        Self {
            rng: rngs::StdRng::from_entropy(),
            move_map: HashMap::new(),
        }
    }
}

impl OpeningBook {
    pub fn load_from_file(path: &str) -> OpeningBook {
        let f = fs::read_to_string(path).unwrap();
        let mut lines: std::str::Lines<'_> = f.lines();

        let mut map: HashMap<u64, (Vec<ChessMove>, Vec<i32>)> = HashMap::new();

        let mut gs = GameState::start_position();

        let mut tree = Vec::new();
        let mut index = 0;
        while let Some(mut line) = lines.next() {
            index += 1;

            let trimed = line.trim();
            if trimed == "" {
                // println!("Ignoring empty line {}", line);
                continue;
            }

            if trimed == "]" {
                for _ in 0..tree
                    .pop()
                    .expect(&format!("Found unclosed ] at line {}", index))
                {
                    gs.undo_move();
                }
                continue;
            }

            if !line.starts_with(&"\t".repeat(tree.len())) {
                panic!(
                    "Incorrect indentation at line {index} expected {} tabs",
                    tree.len()
                )
            }

            if trimed.starts_with("//") {
                // println!("Ignoring comment {}", line);
                continue;
            }

            if line.starts_with("fen") {
                gs = GameState::from_fen(&line[4..]);
                continue;
            }

            line = line.trim();

            let mut split = line.split(" ");

            let mut variation_length = 0;
            let mut open_new_var = false;
            while let Some(ms) = split.next() {
                if ms == "[" {
                    open_new_var = true;
                    break;
                }

                let all_moves = gs.gen_legal_moves();

                let mut res_cm = None;
                for &m in &all_moves {
                    if m.san_move(&all_moves) == ms {
                        res_cm = Some(m);
                    }
                }

                if let Some(cm) = res_cm {
                    variation_length += 1;

                    let hash = gs.zobrist_hash.hash;
                    if let Some((list, enc)) = map.get_mut(&hash) {
                        list.push(cm);
                        list.sort_by(|x, y| {
                            x.uci_move().to_string().cmp(&y.uci_move().to_string())
                        });

                        let prev_size = list.len();
                        list.dedup();

                        if list.len() != prev_size {
                            println!("{}", gs.to_pgn("white", "black"));
                            gs.board_state.piece_board.print();

                            panic!("Trying to add redundant move {} at line {index} previously encountered at {:?}", cm.san_move(&all_moves), enc)
                        }
                    } else {
                        map.insert(hash, (vec![cm], vec![index]));
                    }

                    gs.make_move(cm);

                    if let Some((_, enc)) = map.get_mut(&gs.zobrist_hash.hash) {
                        enc.push(index);
                    }
                } else {
                    println!("{}", gs.to_pgn("white", "black"));
                    gs.board_state.piece_board.print();

                    panic!(
                        "Could not parse move [{ms}] at line {index} options are: {}",
                        all_moves
                            .iter()
                            .map(|m| m.san_move(&all_moves))
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                }
            }

            if open_new_var {
                tree.push(variation_length);
            } else {
                for _ in 0..variation_length {
                    gs.undo_move();
                }
            }
        }

        return OpeningBook {
            move_map: map,
            ..Default::default()
        };
    }

    pub fn collect_all_positions(&self) -> Vec<GameState> {
        let mut list = Vec::new();

        let mut gs = GameState::start_position();

        try_all_pos(&mut gs, &self, &mut list);

        return list;

        fn try_all_pos(gs: &mut GameState, book: &OpeningBook, ret: &mut Vec<GameState>) {
            ret.push(gs.clone());

            if let Some((ml, _)) = book.move_map.get(&gs.zobrist_hash.hash) {
                if ml.len() == 0 {
                    println!("kek");
                }
                for &m in ml {
                    gs.make_move(m);

                    try_all_pos(gs, book, ret);

                    gs.undo_move();
                }
            }
        }
    }

    pub fn check_for_unbalanced_position(&self) {
        let all_pos = self.collect_all_positions();

        all_pos.par_chunks(500).for_each(|list| {
            let mut sf = StockFishBot {
                max_time: Some(1000),
                ..Default::default()
            };

            for gs in list {
                let (_, eval) = sf.get_best_move(&mut gs.clone());

                if eval.abs() > 110 {
                    println!("Uneven position {eval}");
                    println!("{}", gs.to_pgn("white", "black"));
                }
            }
        });
    }

    pub fn get_random_move(&mut self, game_state: &GameState) -> Option<ChessMove> {
        if let Some((list, _)) = self.move_map.get(&game_state.zobrist_hash.hash) {
            return Some(list[self.rng.gen_range(0..list.len())]);
        } else {
            return None;
        }
    }
}
