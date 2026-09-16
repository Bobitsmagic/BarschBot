use crate::{game::game_state::GameState, moves::chess_move::ChessMove};

use super::{search_functions::bb_timed_search, search_stats::SearchStats, settings::Settings};

#[derive(Clone)]
pub struct Barschbot {
    pub name: String,
    pub settings: Settings,
    search_stats: SearchStats,
}

impl Barschbot {
    pub fn named(settings: Settings, name: String) -> Barschbot {
        Barschbot {
            name,
            settings,
            search_stats: SearchStats::new(),
        }
    }

    pub fn new(settings: Settings) -> Barschbot {
        Barschbot::named(settings, String::from("Barschbot"))
    }

    pub fn search_time(&mut self, game_state: &mut GameState, time_left: u128) -> ChessMove {
        let (bm, _, stats) = bb_timed_search(game_state, time_left, i32::MAX, &self.settings);

        self.search_stats += stats;

        return bm;
    }

    pub fn search_depth(&mut self, game_state: &mut GameState, max_depth: i32) -> (ChessMove, i32) {
        let (bm, eval, stats) = bb_timed_search(game_state, u128::MAX, max_depth, &self.settings);

        self.search_stats += stats;

        return (bm, eval);
    }
}
