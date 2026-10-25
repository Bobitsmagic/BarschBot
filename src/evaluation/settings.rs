use crate::{
    board::player_color::PlayerColor::{Black, White},
    evaluation::{
        hans_eval::{self, EvaluationSettings},
        move_sorting::{generic_move_sorter, QuietMoveTable},
        settings::MoveOrderingMode::BetterMoveSorter,
        wiesel_eval::{self, WieselSettings},
    },
    game::{board_state::BoardState, game_state::GameState},
    moves::{chess_move::ChessMove, move_gen::MoveVector},
};

#[derive(Debug, Clone, Copy)]
pub enum EvaluationMode {
    HansEvaluation(hans_eval::EvaluationSettings),
    WieselEvaluation(WieselSettings),
}

#[derive(Debug, Clone, Copy)]
pub enum MoveOrderingMode {
    QuietMove,
    StaticExchange,
    BetterMoveSorter,
    GenericMoveSorter(fn(&mut MoveVector, &mut BoardState, ChessMove, &QuietMoveTable) -> ()),
}

#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub time_percentage: f32,
    pub quiessence_depth: i32,
    pub null_move_pruning: i32,
    pub check_extensions: i32,
    pub move_ordering_mode: MoveOrderingMode,
    pub quiessence_ordering_mode: MoveOrderingMode,
    pub evaluation_mode: EvaluationMode,
    pub use_endgame_table: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            time_percentage: 0.015,
            quiessence_depth: 5,
            check_extensions: 0,
            null_move_pruning: 0,
            move_ordering_mode: MoveOrderingMode::GenericMoveSorter(
                generic_move_sorter::<true, true, 2, true, true>,
            ),
            quiessence_ordering_mode: MoveOrderingMode::GenericMoveSorter(
                generic_move_sorter::<true, false, 1, true, false>,
            ),
            evaluation_mode: EvaluationMode::HansEvaluation(EvaluationSettings {
                use_new_feature: false,
                attr_weights: hans_eval::STANDARD_EVAL,
            }),
            use_endgame_table: true
        }
    }
}

impl Settings {
    pub fn evaluate(&self, game_state: &GameState, alpha: i32, beta: i32) -> i32 {
        return match self.evaluation_mode {
            EvaluationMode::HansEvaluation(attr) => {
                let (min_value, max_value) = match game_state.active_color() {
                    White => (alpha, beta),
                    Black => (-beta, -alpha),
                };

                hans_eval::evaluation_function(game_state, &attr, min_value, max_value)
            }
            EvaluationMode::WieselEvaluation(set) => {
                wiesel_eval::evaluation_function(game_state, &set)
            }
        };
    }
}
