

use crate::{board::{dynamic_state::DynamicState, piece_board::PieceBoard, player_color::PlayerColor, zobrist_hash::ZobristHash}, fen::fen_helper, moves::{chess_move::ChessMove, move_gen, pseudo_move_gen}};

use super::{board_state::BoardState, game_flags::GameFlags};

#[derive(Clone, PartialEq, Eq)]
pub struct GameState {
    pub board_state: BoardState,
    pub zobrist_hash: ZobristHash,
    pub move_stack: Vec<ChessMove>,
    pub flag_stack: Vec<GameFlags>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            board_state: BoardState::empty(),
            move_stack: Vec::new(),
            flag_stack: vec![GameFlags::start_flags()],
            zobrist_hash: ZobristHash::empty(),
        }
    }

    pub fn start_position() -> GameState {
        GameState {
            board_state: BoardState::start_position(),
            move_stack: Vec::new(),
            flag_stack: vec![GameFlags::start_flags()],
            zobrist_hash: ZobristHash::from_position(&PieceBoard::start_position(), GameFlags::start_flags()),
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

    pub fn from_fen(fen: &str) -> GameState {
        let (pb, flags) = fen_helper::from_fen(fen);

        GameState {
            board_state: BoardState::from_piece_board(&pb),
            move_stack: Vec::new(),
            flag_stack: vec![flags],
            zobrist_hash: ZobristHash::from_position(&pb, flags),
        }
    }

    pub fn make_move(&mut self, m: ChessMove) {
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
        let m = self.move_stack.pop().unwrap();
        self.board_state.undo_move(m);
        self.zobrist_hash.undo_move(m);

        let top_flag = self.flag_stack.pop().unwrap();

        self.zobrist_hash.toggle_flags(top_flag); //Remove latest
        self.zobrist_hash.toggle_flags(*self.flag_stack.last().unwrap()); //Add old flags
    }

    pub fn gen_legal_moves(&self) -> Vec<ChessMove> {
        return move_gen::gen_legal_moves_bitboard(&self.board_state, &self.get_flags());
        // return pseudo_move_gen::gen_legal_moves_bitboard(&self.board_state, self.flag_stack.last().unwrap());
    }
}