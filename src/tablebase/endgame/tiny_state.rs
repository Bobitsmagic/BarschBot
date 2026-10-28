use arrayvec::ArrayVec;

use crate::{
    board::{
        bit_array::BitArray,
        bit_board::BitBoard,
        dynamic_state::DynamicState,
        piece_board::PieceBoard,
        player_color::PlayerColor,
        square::{self, Square},
    },
    game::{
        board_state::BoardState,
        game_flags::{self, GameFlags},
        game_result::{DrawType, GameResult, WinType},
        game_state::GameState,
    },
    moves::{
        chess_move::ChessMove,
        move_gen::{self, MoveVector},
    },
};
use std::hash::Hash;
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hasher},
};

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TinyState {
    pub board: BitBoard,
    pub active_color: PlayerColor,
    pub ep_square: i8,
}

impl Debug for TinyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n{}\n Active: {:?} EpSquare {} Hash {:x}",
            self.board.to_string(),
            self.active_color,
            self.ep_square.square_string(),
            self.get_hash_value()
        )
    }
}

type FlagList = ArrayVec<TinyState, 8>;

impl TinyState {
    pub fn gen_legal_gameflags(bb: &BitBoard) -> FlagList {
        let mut active_colors: ArrayVec<PlayerColor, 2> = ArrayVec::new();

        active_colors.push(PlayerColor::White);
        active_colors.push(PlayerColor::Black);
        active_colors.retain(|x| !bb.is_in_check(!*x));

        let mut ret = ArrayVec::new();
        for active_color in active_colors {
            ret.push(TinyState {
                board: bb.clone(),
                active_color,
                ep_square: square::NONE,
            });

            for ep_square in bb.get_valid_ep_squares(active_color).iterate_squares() {
                ret.push(TinyState {
                    board: bb.clone(),
                    active_color,
                    ep_square,
                });
            }
        }

        return ret;
    }

    pub fn from_game_state(gs: &GameState) -> TinyState {
        let flags = gs.get_flags();
        TinyState {
            board: gs.board_state.bit_board.clone(),
            active_color: flags.active_color,
            ep_square: flags.en_passant_square,
        }
    }

    pub fn get_game_result(&self) -> (GameResult, MoveVector) {
        let flags = GameFlags {
            active_color: self.active_color,
            en_passant_square: self.ep_square,
            ..game_flags::EMPTY_FLAGS
        };

        let piece_board = PieceBoard::from_bit_board(&self.board);
        let board_state = BoardState {
            piece_board,
            bit_board: self.board.clone(),
        };

        let (moves, check) = move_gen::gen_legal_moves_check_pins(&board_state, &flags);

        let result = match (moves.is_empty(), check) {
            (true, true) => GameResult::Win(!self.active_color, WinType::Checkmate),
            (true, false) => GameResult::Draw(DrawType::StaleMate),
            _ => GameResult::Undecided,
        };

        return (result, moves);
    }

    pub fn make_move(&mut self, m: ChessMove) {
        self.board.make_move(m);
        self.active_color = !self.active_color;

        if m.move_piece.is_pawn()
            && m.start.rank().abs_diff(m.end.rank()) == 2
            && self.board.pawn_has_neighbour(m.move_piece.color(), m.end)
        {
            self.ep_square = match m.move_piece.color() {
                PlayerColor::White => m.end.down(),
                PlayerColor::Black => m.end.up(),
            };
        } else {
            self.ep_square = square::NONE;
        }
    }

    pub fn get_hash_value(&self) -> u64 {
        assert!(self.ep_square >= 0 && self.ep_square <= 64);

        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }

    pub fn flip_x(&self) -> TinyState {
        TinyState {
            board: self.board.flip_x(),
            active_color: self.active_color,
            ep_square: if self.ep_square.is_valid_square() {
                self.ep_square.flip_x()
            } else {
                square::NONE
            },
        }
    }

    pub fn flip_y(&self) -> TinyState {
        assert!(!self.ep_square.is_valid_square());

        TinyState {
            board: self.board.flip_y(),
            active_color: self.active_color,
            ep_square: square::NONE,
        }
    }

    pub fn transpose(&self) -> TinyState {
        assert!(!self.ep_square.is_valid_square());
        TinyState {
            board: self.board.transpose(),
            active_color: self.active_color,
            ep_square: square::NONE,
        }
    }

    pub fn flip_color(&self) -> TinyState {
        TinyState {
            board: self.board.flip_color(),
            active_color: !self.active_color,
            ep_square: if self.ep_square.is_valid_square() {
                self.ep_square.flip_y()
            } else {
                square::NONE
            },
        }
    }

    pub fn print(&self) {
        println!("{:?}", self);
    }

    //TODO: use ArrayVec
    pub fn find_representative(&self) -> TinyState {
        let mut all_boards: ArrayVec<TinyState, 16> = ArrayVec::new();

        all_boards.push(self.clone());
        all_boards.push(self.flip_x());

        for i in 0..all_boards.len() {
            all_boards.push(all_boards[i].flip_color());
        }

        if self.board.pawn == 0 {
            for i in 0..all_boards.len() {
                all_boards.push(all_boards[i].flip_y());
            }

            for i in 0..all_boards.len() {
                all_boards.push(all_boards[i].transpose());
            }
        }

        return all_boards.into_iter().min().unwrap();
    }
}

#[cfg(test)]
mod small_vec_test {
    use crate::{
        board::{
            bit_board::BitBoard, piece_type::ColoredPieceType, player_color::ALL_COLORS, square::*,
        },
        tablebase::endgame::tiny_state::TinyState,
    };

    #[test]
    fn test_identity() {
        for active_color in ALL_COLORS {
            for s1 in VALID_SQUARES {
                for s2 in VALID_SQUARES {
                    if s1 == s2 {
                        continue;
                    }

                    let mut board = BitBoard::empty();
                    board.add_piece(ColoredPieceType::WhiteKing, s1);
                    board.add_piece(ColoredPieceType::BlackKing, s2);

                    let ts = TinyState {
                        board,
                        active_color,
                        ep_square: NONE,
                    };

                    assert_eq!(ts, ts.flip_x().flip_x());
                    assert_eq!(ts, ts.flip_y().flip_y());
                    assert_eq!(ts, ts.transpose().transpose());
                    assert_eq!(ts, ts.flip_color().flip_color());
                    assert_eq!(ts, ts.flip_x().flip_y().flip_x().flip_y());
                }
            }
        }
    }
}
