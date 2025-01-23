use std::default;

use crate::{board::piece_board::PieceBoard, moves::chess_move::{self, ChessMove}};

pub const ANIMATION_TIME: f64 = 10.0;

pub struct RenderState {
    pub piece_board: PieceBoard,
    pub lm: ChessMove,
    pub flip: bool,
    pub animation_time: f64,
    pub white_time: u128,
    pub black_time: u128,
    pub white_name: String,
    pub black_name: String,
}

impl default::Default for RenderState {
    fn default() -> Self {
        RenderState {
            piece_board: PieceBoard::start_position(),
            lm: chess_move::NULL_MOVE,
            flip: false,
            animation_time: ANIMATION_TIME,
            white_time: 0,
            black_time: 0,
            white_name: String::from("White player"),
            black_name: String::from("Black player"),
        }
    }
}

impl RenderState {
    pub fn new() -> Self {
        RenderState::default()
    }

    pub fn render_move(piece_board: PieceBoard, lm: ChessMove, flip: bool) -> Self {
        RenderState {
            piece_board,
            lm,
            flip,
            
            ..Default::default()
        }
    }

    pub fn animate(piece_board: PieceBoard, lm: ChessMove, flip: bool) -> Self {
        RenderState {
            piece_board,
            lm,
            flip,
            animation_time: 0.0,
            
            ..Default::default()
        }
    }

    pub fn render_move_timed(piece_board: PieceBoard, lm: ChessMove, flip: bool, white_time: u128, black_time: u128) -> Self {
        RenderState {
            piece_board,
            lm,
            flip,
            animation_time: ANIMATION_TIME,
            white_time,
            black_time,

            ..Default::default()
        }
    }

    pub fn render_move_named(piece_board: PieceBoard, lm: ChessMove, flip: bool, white_time: u128, black_time: u128, white_name: String, black_name: String) -> Self {
        RenderState {
            piece_board,
            lm,
            flip,
            animation_time: ANIMATION_TIME,
            white_time,
            black_time,
            white_name,
            black_name,
            
            ..Default::default()
        }
    }
}