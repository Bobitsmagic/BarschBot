use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Instant,
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    board::bit_board::BitBoard,
    evaluation::search_functions::CHECKMATE_VALUE,
    game::{
        game_result::{DrawType, GameResult},
        game_state::GameState,
    },
    tablebase::endgame::{
        board_generation::{gen_endgame, generate_king_boards_sym, generate_king_boards_sym_color},
        endgame_type::{self, EndGameType},
        tiny_state::TinyState,
    },
};

type Entry = HashMap<u64, i16>;

const CHECKMATE_VAL: i16 = 1000;

pub struct EndGameTable {
    max_count: usize,
    map: HashMap<EndGameType, Entry>,
}

impl EndGameTable {
    pub fn get_empty() -> EndGameTable {
        EndGameTable {
            max_count: 0,
            map: HashMap::new(),
        }
    }

    pub fn count(&self) -> usize {
        self.map.values().map(|x| x.len()).sum()
    }

    pub fn longest_sequence(&self) -> i16 {
        let mut longest = i16::MAX;

        for m in self.map.values() {
            for &v in m.values() {
                if v != 0 && v.abs() < longest {
                    longest = v.abs();
                }
            }
        }

        return CHECKMATE_VAL - longest;
    }

    pub fn get_eval(&self, gs: &GameState) -> Option<i32> {
        let pc = gs.board_state.piece_count();

        if pc as usize > self.max_count + 2 {
            return None;
        }

        let ts = TinyState::from_game_state(gs).find_representative();
        let hash = ts.get_hash_value();

        let egt = ts.board.get_endgame_type().representative();

        let eval = *self
            .map
            .get(&egt)
            .expect(&format!(
                "Could not find endgame type: {} -> {:?} board: \n{:?}",
                egt.to_string(),
                egt.pieces,
                ts
            ))
            .get(&hash)
            .expect(&format!(
                "Could not find position in previous type table {:?}",
                ts
            ));

        if eval != 0 {
            return Some(
                (CHECKMATE_VALUE - (CHECKMATE_VAL - eval.abs()) as i32) * eval.signum() as i32,
            );
        } else {
            return Some(0);
        }
    }

    pub fn generate(max_piece_count: usize) -> EndGameTable {
        let mut table = EndGameTable {
            max_count: max_piece_count,
            map: HashMap::new(),
        };

        let list = endgame_type::generate_piece_lists_sym(max_piece_count);
        let kl = generate_king_boards_sym();
        let kl_sym = generate_king_boards_sym_color();

        for et in list {
            println!("{}", et.to_string());
            let entry = table.generate_endgame_dyn(et.representative(), &kl, &kl_sym);
            table.map.insert(et.representative(), entry);

            println!("Longest sequence: {}", table.longest_sequence())
        }

        return table;
    }

    pub fn generate_par(max_piece_count: usize) -> EndGameTable {
        rayon::ThreadPoolBuilder::new()
            .num_threads(3)
            .build_global()
            .unwrap();


        let mut table = EndGameTable {
            max_count: max_piece_count,
            map: HashMap::new(),
        };

        let mut open_endgames = endgame_type::generate_piece_lists_sym(max_piece_count);
        let kl = generate_king_boards_sym();
        let kl_sym = generate_king_boards_sym_color();

        let mut visited_endgames: Vec<EndGameType> = Vec::new();

        let start_time = Instant::now();
        while open_endgames.len() > 0 {
            let mut next_endgames = Vec::new();

            for i in (0..open_endgames.len()).rev() {
                let all_suc = open_endgames[i].gen_successors();
                let mut complete = true;
                // println!("Next?: {}", open_endgames[i].to_string());
                for suc in all_suc {
                    if !visited_endgames.contains(&suc.representative()) {
                        // println!("no because {} is missing {:?}", suc.representative().to_string(), suc.representative().pieces);

                        complete = false;
                        break;
                    }
                }

                if complete {
                    next_endgames.push(open_endgames.remove(i).representative());
                }
            }

            println!("Next: ");
            for egt in &next_endgames {
                println!("{}", egt.to_string());
            }
            println!("###########");

            let next_entries = Arc::new(Mutex::new(Vec::new()));
            next_endgames.par_iter().for_each(|egt| {
                let entry = table.generate_endgame_dyn(egt.representative(), &kl, &kl_sym);
                next_entries
                    .lock()
                    .unwrap()
                    .push((egt.representative(), entry));
                // println!("{}", egt.to_string());
            });

            let mut list = next_entries.lock().unwrap();

            while let Some((egt, entry)) = list.pop() {
                table.map.insert(egt.representative(), entry);
                visited_endgames.push(egt.representative());
            }

            println!("Longest sequence: {}", table.longest_sequence());
            println!("Overall time: {:?}", start_time.elapsed());
        }

        return table;
    }

    fn generate_endgame_dyn(
        &self,
        et: EndGameType,
        kl: &Vec<BitBoard>,
        kl_sym: &Vec<BitBoard>,
    ) -> Entry {
        let mut open_list = gen_endgame(&et, if et.is_symmetric() { &kl_sym } else { &kl });

        let mut terminal = HashMap::with_capacity(open_list.len());
        let mut next_repr = HashMap::with_capacity(open_list.len());
        let mut lower_bound = HashMap::with_capacity(open_list.len());

        // println!("Depth: 1");
        //Check for checkmate/stalemate
        for i in (0..open_list.len()).rev() {
            let (gs, ml) = open_list[i].get_game_result();
            let hash = open_list[i].get_hash_value();

            // if et != (EndGameType { pieces: [1,0,0,0,0,1,0,0,0,0] }) {
            //     gs = GameResult::Draw(DrawType::InsufficientMaterial);
            // }

            match gs {
                GameResult::Win(_, _) => {
                    terminal.insert(hash, -CHECKMATE_VAL);
                    open_list.swap_remove(i);
                }
                GameResult::Draw(_) => {
                    terminal.insert(hash, 0);
                    open_list.swap_remove(i);
                }
                _ => {
                    let mut nl = Vec::new();
                    let mut max_value = i16::MIN;
                    for m in ml {
                        let mut ts = open_list[i].clone();
                        ts.make_move(m);
                        ts = ts.find_representative();

                        let hash = ts.get_hash_value();
                        if m.is_capture() || m.is_promotion() {
                            //There is an error here with Pp egt being Pp egt after capture is aplied
                            let egt = ts.board.get_endgame_type().representative();
                            let eval = *self.map.get(&egt).expect(&format!("Could not find endgame type: {} -> {:?} after move {} board: \n{:?}", egt.to_string(), egt.pieces, m.to_string(), open_list[i])).get(&hash).expect(&format!("Could not find position in previous type table {:?}", ts));

                            max_value = max_value.max(-eval);
                        } else {
                            nl.push(ts.get_hash_value());
                        }
                    }

                    next_repr.insert(hash, nl);
                    lower_bound.insert(hash, max_value);
                }
            }
        }

        let mut depth = 1;
        while open_list.len() > 0 {
            depth += 1;
            // println!("Depth: {}", depth);
            let prev_count = open_list.len();
            for i in (0..open_list.len()).rev() {
                let hash = open_list[i].get_hash_value();
                let mut max_eval = lower_bound[&hash];
                let mut all_terminal = true;

                let next_list = next_repr
                    .get_mut(&hash)
                    .expect("Could not find next_repr in map");

                for j in (0..next_list.len()).rev() {
                    let next_hash = &next_list[j];

                    if let Some(eval) = terminal.get(next_hash) {
                        max_eval = max_eval.max(-eval);
                        next_list.swap_remove(j);
                    } else {
                        all_terminal = false;
                    }
                }

                lower_bound.insert(hash, max_eval);

                if all_terminal || max_eval > 0 {
                    terminal.insert(hash, max_eval - max_eval.signum());
                    open_list.swap_remove(i);
                }
            }

            if open_list.len() == prev_count {
                println!("Could not resolve endgame for {} fall back to insufficient material for remaining {}/{} positions ", et.to_string(), open_list.len(), open_list.len() + terminal.len());

                for i in 0..open_list.len() {
                    terminal.insert(open_list[i].get_hash_value(), 0);
                }
                open_list.clear();
            }
        }
        println!("{}: Max depth: {}", et.to_string(), depth);

        return terminal;
    }
}
