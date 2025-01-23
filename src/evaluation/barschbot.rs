use crate::{game::game_state::GameState, moves::chess_move::ChessMove};

use super::{search_functions::bb_timed_search, search_stats::SearchStats, settings::Settings};

#[derive(Clone)]
pub struct Barschbot {
    settings: Settings,
    search_stats: SearchStats,    
}

impl Barschbot {
    pub fn new(settings: Settings) -> Barschbot {
        Barschbot {
            settings,
            search_stats: SearchStats::new(),
        }
    }

    pub fn search(&mut self, game_state: &mut GameState, time_left: u128) -> ChessMove {
        let (bm, _, stats) = bb_timed_search(game_state, time_left, &self.settings);

        self.search_stats += stats;

        return bm;
    }
}

