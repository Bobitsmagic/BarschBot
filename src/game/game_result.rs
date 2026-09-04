use crate::board::player_color::PlayerColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawType {
    Repetition,
    InsufficientMaterial,
    StaleMate,
    FiftyMoveRule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WinType {
    Checkmate,
    TimeOut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Win(PlayerColor, WinType),
    Draw(DrawType),
    Undecided,
}
