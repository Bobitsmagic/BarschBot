

use std::collections::{HashMap, HashSet};

use crate::{board::{dynamic_state::DynamicState, piece_board::PieceBoard, player_color::PlayerColor, zobrist_hash::ZobristHash}, fen::fen_helper, moves::{chess_move::ChessMove, move_gen::{self, MoveVector}, move_iterator::MoveIterator}};

use super::{board_state::BoardState, game_flags::GameFlags, game_result::GameResult};

#[derive(Clone)]
pub struct GameState {
    pub board_state: BoardState,
    pub zobrist_hash: ZobristHash,
    pub visited_pos: HashSet<u64>,
    pub move_stack: Vec<ChessMove>,
    pub flag_stack: Vec<GameFlags>,
    pub legal_moves: Option<MoveVector>,
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.board_state == other.board_state &&
        self.zobrist_hash == other.zobrist_hash &&
        self.move_stack == other.move_stack &&
        self.flag_stack == other.flag_stack
    }
}

impl GameState {
    pub fn start_position() -> GameState {
        GameState {
            board_state: BoardState::start_position(),
            move_stack: Vec::new(),
            flag_stack: vec![GameFlags::start_flags()],
            zobrist_hash: ZobristHash::from_position(&PieceBoard::start_position(), GameFlags::start_flags()),
            legal_moves: None,
            visited_pos: HashSet::new(),
        }
    }

    pub fn get_flags(&self) -> GameFlags {
        return *self.flag_stack.last().unwrap();
    }

    pub fn last_move(&self) -> Option<ChessMove> {
        return self.move_stack.last().cloned();
    }

    pub fn active_color(&self) -> PlayerColor {
        return self.flag_stack.last().unwrap().active_color;
    }

    pub fn to_fen(&self) -> String {
        return fen_helper::to_fen(&self.board_state.piece_board, &self.get_flags());
    }

    pub fn from_fen(fen: &str) -> GameState {
        let (pb, flags) = fen_helper::from_fen(fen);

        GameState {
            board_state: BoardState::from_piece_board(&pb),
            move_stack: Vec::new(),
            flag_stack: vec![flags],
            zobrist_hash: ZobristHash::from_position(&pb, flags),
            legal_moves: None,
            visited_pos: HashSet::new(),
        }
    }

    pub fn make_move(&mut self, m: ChessMove) {
        self.legal_moves = None;

        if !self.visited_pos.insert(self.zobrist_hash.hash) {
            self.board_state.piece_board.print();
            panic!("Repetition detected");
        }

        self.board_state.make_move(m);
        self.move_stack.push(m);
        let mut new_flags = (*self.flag_stack.last().unwrap()).clone();

        self.zobrist_hash.make_move(m);
        self.zobrist_hash.toggle_flags(new_flags); //Remove old flags
        new_flags.make_move(m, &self.board_state.bit_board);
        self.zobrist_hash.toggle_flags(new_flags); //Add new flags


        self.flag_stack.push(new_flags);
    }

    pub fn undo_move(&mut self) {
        self.legal_moves = None;

        let m = self.move_stack.pop().unwrap();
        self.board_state.undo_move(m);
        self.zobrist_hash.undo_move(m);

        let top_flag = self.flag_stack.pop().unwrap();

        self.zobrist_hash.toggle_flags(top_flag); //Remove latest
        self.zobrist_hash.toggle_flags(*self.flag_stack.last().unwrap()); //Add old flags

        if !self.visited_pos.remove(&self.zobrist_hash.hash) {
            self.board_state.piece_board.print();
            panic!("Previous position not found");
        }
    }

    pub fn gen_legal_moves(&mut self) -> MoveVector {
        if self.legal_moves.is_none() {
            self.legal_moves = Some(move_gen::gen_legal_moves(&self.board_state, &self.get_flags()));
        }

        return self.legal_moves.as_ref().unwrap().clone();
    }
    pub fn gen_legal_moves_iterator(&self) -> MoveIterator {
        return move_gen::gen_legal_moves_iterator(&self.board_state, &self.get_flags());
    }

    pub fn game_result(&self) -> GameResult {
        if self.visited_pos.contains(&self.zobrist_hash.hash) {
            // self.board_state.piece_board.print();
            return GameResult::Draw;
        }

        move_gen::game_result(&self.board_state, &self.get_flags())
    }
}