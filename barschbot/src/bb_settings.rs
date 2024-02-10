use crate::evaluation::{EvalAttributes, EvalAttributes2};

#[derive(Clone)]
pub struct BBSettings {
    pub max_depth: u8,
    pub max_quiescence_depth: u8,
    pub end_game_table: bool,
    pub null_move_pruning: bool,
    pub null_move_pruning_margin: f32,
    pub null_move_pruning_depth: u8,
    pub max_extensions: u8,
    pub eval_factors: EvalFactorsFloat,
    pub min_search_time: u64
}

pub const STANDARD_BB_SETTINGS: BBSettings = BBSettings { 
    max_depth: 6, 
    max_quiescence_depth: 3, 
    max_extensions: 2, 
    end_game_table: true, 
    null_move_pruning: true,
    null_move_pruning_margin: 0.3,
    null_move_pruning_depth: 3, 
    min_search_time: 0, 
    eval_factors: STANDARD_EVAL_FACTORS 
};

#[derive(Debug, Copy, Clone)]
pub enum FactorName {
    PieceValueP, PieceValueN, PieceValueB, PieceValueR, PieceValueQ,
    SafeMobilityP, SafeMobilityN, SafeMobilityB, SafeMobilityR, SafeMobilityQ, SafeMobilityK,
    UnsafeMobilityP, UnsafeMobilityN, UnsafeMobilityB, UnsafeMobilityR, UnsafeMobilityQ, UnsafeMobilityK,

    LateFactorRange,
    SquareControl,

    PawnRank2, PawnRank3, PawnRank4, PawnRank5, PawnRank6, PawnRank7,
    PassedPawn,
    DoubledPawn,
    IsolatedPawn,

    KnightOutpost,

    KingExposed,
    KingControl,
    SafeCheck,
    UnsafeCheck,
}

pub const ALL_NAMES: [FactorName; 33] = [
    FactorName::PieceValueP, FactorName::PieceValueN, FactorName::PieceValueB, FactorName::PieceValueR, FactorName::PieceValueQ,
    FactorName::SafeMobilityP, FactorName::SafeMobilityN, FactorName::SafeMobilityB, FactorName::SafeMobilityR, FactorName::SafeMobilityQ, FactorName::SafeMobilityK,
    FactorName::UnsafeMobilityP, FactorName::UnsafeMobilityN, FactorName::UnsafeMobilityB, FactorName::UnsafeMobilityR, FactorName::UnsafeMobilityQ, FactorName::UnsafeMobilityK,

    FactorName::LateFactorRange,
    FactorName::SquareControl,

    FactorName::PawnRank2, FactorName::PawnRank3, FactorName::PawnRank4, FactorName::PawnRank5, FactorName::PawnRank6, FactorName::PawnRank7,
    FactorName::PassedPawn,
    FactorName::DoubledPawn,
    FactorName::IsolatedPawn,

    FactorName::KnightOutpost,

    FactorName::KingExposed,
    FactorName::KingControl,
    FactorName::SafeCheck,
    FactorName::UnsafeCheck,
];



#[derive(Clone)]
pub struct EvalFactorsFloat {
    values: [f32; 33],
}

pub const STANDARD_EVAL_FACTORS: EvalFactorsFloat = EvalFactorsFloat {
    values: [
        //Piece value
        1.0, 2.8, 3.2, 5.0, 11.0,
        //Safe mobility 
        0.01, 0.0618192, 0.07, 0.053, 0.005, 0.106,
        //Unsafe Mobility
        -0.01, -0.06, -0.02, -0.03, -0.09, -0.07,

        //Late factor range
        0.01,
        //Square control
        0.0106,

        //Pawn push bonus
        -0.062, 0.05, 0.077, 0.1, 0.15, 0.5,
        //Passed pawn value
        0.204, 
        //Doubled pawn penalty
        -0.15, 
        //Isolated pawn penalty
        -0.15,

        //Knight outpost value
        0.062,

        //King exposed penalty
        -0.0066,
        //King control penalty
        -0.162,
        //Safe check value
        0.2,
        //Unsafe check value 
        0.086,
    ]
};

pub const MAX_MATERIAL_SUM: i32 = 3 * 8 + 5 * 4 + 9 * 2;
impl EvalFactorsFloat {
    pub fn evaluate(&self, attributes: &EvalAttributes) -> f32 {
        let values = self.values;
        
        let mut sum = 0.0;
        const START_MAT_SUM: f32 = MAX_MATERIAL_SUM as f32;

        let late_factor = 1.0 + self.get_value(FactorName::LateFactorRange) - attributes.material_sum as f32 / START_MAT_SUM * self.get_value(FactorName::LateFactorRange);

        for i in 0..5 {
            sum += self.get_array(FactorName::PieceValueP, i) * attributes.piece_dif[i] as f32;
        }

        sum *= late_factor;

        sum += self.get_value(FactorName::SquareControl) * attributes.sq_control_dif as f32;

        for i in 0..6 {
            sum += self.get_array(FactorName::SafeMobilityP, i) * attributes.safe_mobility_dif[i] as f32;
            sum += self.get_array(FactorName::UnsafeMobilityP, i) * attributes.unsafe_mobility_dif[i] as f32;
        }

        for i in 0..6 {
            sum += self.get_array(FactorName::PawnRank2, i) * attributes.pawn_push_dif[i] as f32;
        }

        sum += self.get_value(FactorName::PassedPawn) * attributes.passed_pawn_dif as f32;
        sum += self.get_value(FactorName::DoubledPawn) * attributes.doubled_pawn_dif as f32;
        sum += self.get_value(FactorName::IsolatedPawn) * attributes.isolated_pawn_dif as f32;
        
        sum += self.get_value(FactorName::KnightOutpost) * attributes.knight_outpost_dif as f32;

        sum += self.get_value(FactorName::KingExposed) * attributes.king_qn_moves_dif as f32;
        sum += self.get_value(FactorName::KingControl) * attributes.king_control_dif as f32;
        sum += self.get_value(FactorName::SafeCheck) * attributes.safe_check_dif as f32;
        sum += self.get_value(FactorName::UnsafeCheck) * attributes.unsafe_check_dif as f32;

        return sum;
    }

    pub fn get_value(&self, index: FactorName) -> f32 {
        return self.values[index as usize];
    }

    pub fn set_value(&mut self, index: FactorName, value: f32) {
        self.values[index as usize] = value;
    }

    pub fn get_array(&self, index: FactorName, offset: usize) -> f32 {
        return self.values[index as usize + offset];
    }

    pub fn print_all(&self) {
        println!("Settings: ");
        for f in ALL_NAMES {
            println!("\t{:?} -> {}", f, self.get_value(f));
        }
    }
}