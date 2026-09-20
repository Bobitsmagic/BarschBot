use core::panic;
use std::{collections::HashMap, fs, str::Lines};

use rand::{Rng, SeedableRng, rngs::{self, StdRng}};

use crate::{game::game_state::{self, GameState}, moves::chess_move::ChessMove};

pub struct OpeningBook {
    rng: StdRng,
    move_map: HashMap<u64, Vec<ChessMove>>
}

impl Default for OpeningBook {
    fn default() -> Self {
        Self { rng: rngs::StdRng::from_entropy(), move_map: HashMap::new() }
    }
}

impl OpeningBook {
    pub fn load_from_file(path: &str) -> OpeningBook {
       let f =  fs::read_to_string(path).unwrap();

       let mut lines: std::str::Lines<'_> = f.lines();

        let mut map: HashMap<u64, Vec<ChessMove>> = HashMap::new();

        let mut gs = GameState::start_position();

        let mut tree = Vec::new();
        while let Some(mut line) = lines.next() {
            let trimed = line.trim();
            if trimed == "" {
                // println!("Ignoring empty line {}", line);
                continue;
            }

            if trimed == "]" {
                for _ in 0..tree.pop().expect("Found [ despite not elements in the tree") {
                    gs.undo_move();
                }
                continue;
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
                    if  let Some(list) = map.get_mut(&hash) {
                        list.push(cm);
                        list.sort_by(|x, y| x.uci_move().to_string().cmp(&y.uci_move().to_string()));

                        let prev_size = list.len();
                        list.dedup();

                        if list.len() != prev_size {
                            panic!("Trying to add redundant move {} to position: {}", cm.san_move(&all_moves), gs.to_fen())
                        }
                    }
                    else {
                        map.insert(hash, vec![cm]);
                    }

                    gs.make_move(cm);
                }
                else {
                    panic!("Could not parse move [{}] options are: {}", ms, all_moves.iter().map(|m| m.san_move(&all_moves)).collect::<Vec<_>>().join(" "))
                }
            }

            if open_new_var {
                tree.push(variation_length);
            }
            else {
                for _ in 0..variation_length {
                    gs.undo_move();
                }
            }
        }

        return OpeningBook { move_map: map, ..Default::default()}
    }

    pub fn get_move(&mut self, game_state: &GameState) -> Option<ChessMove> {
         if let Some(list) = self.move_map.get(&game_state.zobrist_hash.hash) {
            return Some(list[self.rng.gen_range(0..list.len())]);
         }
         else {
            return None;
         }
    }
}