use std::{collections::HashMap, time::Instant};

use crate::{board::bit_board::BitBoard, game::game_result::GameResult, tablebase::endgame::{array_endgame::ArrayEndgame, board_generation::{gen_all_stats, generate_king_boards_sym, generate_king_boards_sym_color}, compact_endgame::CompactEndgame, endgame_table::TABLE_CHECKMATE_VALUE, endgame_type::{self, EndGameType}}};

pub struct CompactTable {
    max_piece_count: usize,
    map: HashMap<EndGameType, ArrayEndgame>
}

impl CompactTable {
    pub fn generate(max_piece_count: usize) -> CompactTable {
        let mut table = CompactTable {
            max_piece_count,
            map: HashMap::new(),
        };
    
        let list = endgame_type::generate_piece_lists_sym(max_piece_count);
        let kl = generate_king_boards_sym();
        let kl_sym = generate_king_boards_sym_color();
        
        let start_time = Instant::now();
        for et in list {
            println!("Start {} at {:?}", et.to_string(), start_time.elapsed());
            let entry = table.generate_endgame(et.representative(), &kl, &kl_sym);
            table.map.insert(et.representative(), entry);
        }
    
        return table;
    }

    fn generate_endgame(
        &self,
        et: EndGameType,
        kl: &Vec<BitBoard>,
        kl_sym: &Vec<BitBoard>,
    ) -> ArrayEndgame {
        let mut open_list = gen_all_stats(&et, if et.is_symmetric() { &kl_sym } else { &kl });

        let mut terminal = HashMap::with_capacity(open_list.len());
        
        // println!("Depth: 1");
        //Check for checkmate/stalemate
        for i in (0..open_list.len()).rev() {
            let (gs, _) = open_list[i].get_game_result();
            let hash = open_list[i].get_hash_value();

            match gs {
                GameResult::Win(_, _) => {
                    terminal.insert(hash, -TABLE_CHECKMATE_VALUE);
                    open_list.swap_remove(i);
                }
                GameResult::Draw(_) => {
                    terminal.insert(hash, 0);
                    open_list.swap_remove(i);
                }
                _ => { }
            }
        }

        let mut depth = 1;
        while open_list.len() > 0 {
            depth += 1;
            println!("Depth: {}", depth);
            let prev_count = open_list.len();
            for i in (0..open_list.len()).rev() {
                let mut max_value = i16::MIN;
                let mut all_terminal = true;
                for m in open_list[i].get_game_result().1 {
                    let mut ts = open_list[i].clone();
                    ts.make_move(m);
                    ts = ts.find_representative();

                    let hash = ts.get_hash_value();
                    if m.is_capture() || m.is_promotion() {
                        //There is an error here with Pp egt being Pp egt after capture is aplied
                        let egt_repr = ts.board.get_endgame_type().representative();
                        let eval = self.map
                            .get(&egt_repr)
                            .expect(&format!("Could not find endgame type: {} -> {:?} after move {} board: \n{:?}", egt_repr.to_string(), egt_repr.pieces, m.to_string(), open_list[i]))
                            .get(hash).expect(&format!("Could not find position in previous type table {:?}", ts));

                        max_value = max_value.max(-eval);
                    } else {
                        if let Some(eval) = terminal.get(&hash) {
                            max_value = max_value.max(-eval);
                        }
                        else {
                            all_terminal = false;
                        }
                    }
                }

                if all_terminal || max_value > 0 {
                    if terminal.insert(open_list[i].get_hash_value(), max_value - max_value.signum()).is_some() {
                        println!("Collision on state: {:?}", open_list[i])
                    }
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

        return ArrayEndgame::from_hashmap(terminal);
    }
}
