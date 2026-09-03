use crate::{
    evaluation::{
        hans_eval::{self},
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
    pub check_extensions: i32,
    pub evaluation_mode: EvaluationMode,
}

impl Settings {
    pub fn evaluate(&self, game_state: &GameState) -> i32 {
        return match self.evaluation_mode {
            EvaluationMode::HansEvaluation(attr) => {
                hans_eval::evaluation_function(game_state, &attr)
            }
            EvaluationMode::WieselEvaluation(set) => {
                wiesel_eval::evaluation_function(game_state, set)
            }
        };
    }
}
