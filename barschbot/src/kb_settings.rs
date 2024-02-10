use crate::evaluation::{EvalAttributes, EvalAttributes2};

#[derive(Clone)]
pub struct KBSettings {
    pub max_depth: u8,
    pub end_game_table: bool,
    pub null_move_pruning: bool,
    pub eval_factors: EvalFactorsInt,
    pub min_search_time: u64
}

pub const STANDARD_KB_SETTINGS: KBSettings = KBSettings { 
    max_depth: 6, 
    end_game_table: true, 
    null_move_pruning: true, 
    min_search_time: 0, 
    eval_factors: STANDARD_EVAL_FACTORS };


#[derive(Debug, Copy, Clone)]
pub enum FactorName {
    PieceValueP, PieceValueN, PieceValueB, PieceValueR, PieceValueQ,
    SafeMobilityP, SafeMobilityN, SafeMobilityB, SafeMobilityR, SafeMobilityQ, SafeMobilityK,
    UnsafeMobilityP, UnsafeMobilityN, UnsafeMobilityB, UnsafeMobilityR, UnsafeMobilityQ, UnsafeMobilityK,

    SquareControl,

    PawnRank2, PawnRank3, PawnRank4, PawnRank5, PawnRank6, PawnRank7,
    PassedPawn,
    DoubledPawn,
    IsolatedPawn,

    KingExposed,
    KingCaptures,
    SafeCheck,
    UnsafeCheck,
}

pub const ALL_NAMES: [FactorName; 31] = [
    FactorName::PieceValueP, FactorName::PieceValueN, FactorName::PieceValueB, FactorName::PieceValueR, FactorName::PieceValueQ,
    FactorName::SafeMobilityP, FactorName::SafeMobilityN, FactorName::SafeMobilityB, FactorName::SafeMobilityR, FactorName::SafeMobilityQ, FactorName::SafeMobilityK,
    FactorName::UnsafeMobilityP, FactorName::UnsafeMobilityN, FactorName::UnsafeMobilityB, FactorName::UnsafeMobilityR, FactorName::UnsafeMobilityQ, FactorName::UnsafeMobilityK,

    FactorName::SquareControl,

    FactorName::PawnRank2, FactorName::PawnRank3, FactorName::PawnRank4, FactorName::PawnRank5, FactorName::PawnRank6, FactorName::PawnRank7,
    FactorName::PassedPawn,
    FactorName::DoubledPawn,
    FactorName::IsolatedPawn,

    FactorName::KingExposed,
    FactorName::KingCaptures,
    FactorName::SafeCheck,
    FactorName::UnsafeCheck,
];

#[derive(Clone)]
pub struct EvalFactorsInt {
    pub values: [i64; 31],
}

pub const STANDARD_EVAL_FACTORS: EvalFactorsInt = EvalFactorsInt {
    values: [
        1000 | (2000 << 32), //Pawn value
        2800 | (4000 << 32), //Knight value
        3200 | (4300 << 32), //Bishop value
        5000 | (6000 << 32), //Rook value
        11000 | (12000 << 32), //Queen value

        10 | (10 << 32), //Pawn safe mobility
        62 | (60 << 32), //Knight safe mobility
        70 | (50 << 32), //Bishop safe mobility
        8 | (20 << 32), //Rook safe mobility
        5  | (30 << 32), //Queen safe mobility
        1  | (30 << 32), //King safe mobility

        0    | (0 << 32), //Pawn unsafe mobility
        -62  | (-62 << 32), //Knight unsafe mobility
        -70  | (-70 << 32), //Bishop unsafe mobility
        -10  | (-30 << 32), //Rook unsafe mobility 
        -90  | (-20 << 32), //Queen unsafe mobility
        -70  | (-100 << 32), //King unsafe mobility

        10 | (10 << 32), //Square control

        -60 | (-20 << 32), //Pawn rank 2
        50  | (80 << 32), //Pawn rank 3
        77  | (100 << 32), //Pawn rank 4
        100 | (200 << 32), //Pawn rank 5
        150 | (400 << 32), //Pawn rank 6
        500 | (800 << 32), //Pawn rank 7

        100  | (400 << 32), //Passed pawn
        -120 | (-150 << 32), //Doubled pawn
        -100 | (-100 << 32), //Isolated pawn

        -60 | (-10 << 32), //King exposed
        750 | (300 << 32), //King captures
        200 | (300 << 32), //Safe check
        86  | (86 << 32), //Unsafe check
    ]   

};

pub const ZERO_EVAL_FACTORS_INT: EvalFactorsInt = EvalFactorsInt {
    values: [0; 31],
};

impl EvalFactorsInt {
    pub fn evaluate(&self, attributes: &EvalAttributes2) -> i32 {
        let mut result = 0_i64;
        let (eval_vector, mat_sum) = attributes.get_vector();

        for i in 0..self.values.len() {
            result += self.values[i] * eval_vector[i] as i64;
        }
        
        let mat_sum = mat_sum as i32;
        return (result as i32 * mat_sum + (result >> 32) as i32 * (24 - mat_sum)) / 24;
    }
}

