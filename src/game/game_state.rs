use crate::{board::dynamic_state::DynamicState, moves::chess_move::ChessMove};

use super::{board_state::BoardState, game_flags::GameFlags};

pub struct GameState {
    pub board_state: BoardState,
    pub move_stack: Vec<ChessMove>,
    pub flag_stack: Vec<GameFlags>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            board_state: BoardState::empty(),
            move_stack: Vec::new(),
            flag_stack: vec![GameFlags::start_flags()],
        }
    }

    pub fn start_position() -> GameState {
        GameState {
            board_state: BoardState::start_position(),
            move_stack: Vec::new(),
            flag_stack: vec![GameFlags::start_flags()],
        }
    }

    pub fn make_move(&mut self, m: ChessMove) {
        self.board_state.make_move(m);
        self.move_stack.push(m);
        let mut new_flags = (*self.flag_stack.last().unwrap()).clone();
        new_flags.make_move(m);
        self.flag_stack.push(new_flags);
    }

    pub fn undo_move(&mut self) {
        let m = self.move_stack.pop().unwrap();
        self.board_state.undo_move(m);
        self.flag_stack.pop();
    }
}