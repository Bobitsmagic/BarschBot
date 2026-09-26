use crate::{
    board::player_color::PlayerColor::{Black, White},
    evaluation::{
        hans_eval::{self, EvaluationSettings},
        wiesel_eval::{self, WieselSettings},
    },
    game::game_state::GameState,
};

#[derive(Debug, Clone, Copy)]
pub enum EvaluationMode {
    HansEvaluation(hans_eval::EvaluationSettings),
    WieselEvaluation(WieselSettings),
}

#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub time_percentage: f32,
    pub quiessence_depth: i32,
    pub null_move_pruning: i32,
    pub check_extensions: i32,
    pub evaluation_mode: EvaluationMode,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            time_percentage: 0.015,
            quiessence_depth: 5,
            check_extensions: 0,
            null_move_pruning: 0,
            evaluation_mode: EvaluationMode::HansEvaluation(EvaluationSettings {
                use_new_feature: false,
                attr_weights: hans_eval::STANDARD_EVAL,
            }),
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
