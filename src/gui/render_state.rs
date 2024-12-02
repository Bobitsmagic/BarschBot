use crate::{board::piece_board::PieceBoard, moves::chess_move::{self, ChessMove}};

pub const ANIMATION_TIME: f64 = 10.0;

pub struct RenderState {
    pub piece_board: PieceBoard,
    pub lm: ChessMove,
    pub flip: bool,
    pub animation_time: f64,
}

impl RenderState {
    pub fn new() -> Self {
        RenderState {
            piece_board: PieceBoard::start_position(),
            lm: chess_move::NULL_MOVE,
            flip: false,
            animation_time: ANIMATION_TIME,
        }
    }

    pub fn render_move(piece_board: PieceBoard, lm: ChessMove, flip: bool) -> Self {
        RenderState {
            piece_board,
            lm,
            flip,
            animation_time: ANIMATION_TIME,
        }
    }

    pub fn animate(piece_board: PieceBoard, lm: ChessMove, flip: bool) -> Self {
        RenderState {
            piece_board,
            lm,
            flip,
            animation_time: 0.0,
        }
    }
}