#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    WhiteWin,
    BlackWin,
    Draw,
    Undecided,
}