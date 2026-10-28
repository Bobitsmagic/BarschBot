use std::{collections::HashMap};

use crate::{evaluation::search_functions::CHECKMATE_VALUE, game::game_state::GameState, tablebase::endgame::{endgame_table::{self, EndGameTable, TABLE_CHECKMATE_VALUE}, tiny_state::TinyState}};

pub struct CompactEndgame {
    max_piece_count: usize,
    pub map: HashMap<u64, i16>
    
}

impl CompactEndgame {
    pub fn from_endgame_table(endgame: &EndGameTable) -> CompactEndgame {
        let mut map = HashMap::new();

        for (egt, eg) in &endgame.map {
            println!("Starting {}", egt.to_string());
            for (hash, eval) in eg {
                if let Some(prev) = map.insert(*hash, *eval) {
                    println!("Egt: {} Collision at {hash} -> {prev} vs {eval}", egt.to_string());
                    panic!();
                }
            }
        }

        assert_eq!(map.len(), endgame.count());

        return CompactEndgame { max_piece_count: endgame.max_piece_count, map };
    }

    pub fn get(&self, gs: &GameState) -> Option<i32> {
        let pc = gs.board_state.piece_count();

        if pc as usize > self.max_piece_count + 2 {
            return None;
        }

        let ts = TinyState::from_game_state(gs).find_representative();
        let hash = ts.get_hash_value();

        println!("Searching {:?}", ts);
        let eval = *self
            .map
            .get(&hash)
            .expect(&format!(
                "Could not find position in previous type table {:?}",
                ts
            ));


        if eval != 0 {
            return Some(
                (CHECKMATE_VALUE - (TABLE_CHECKMATE_VALUE - eval.abs()) as i32) * eval.signum() as i32,
            );
        } else {
            return Some(0);
        }
    }
}