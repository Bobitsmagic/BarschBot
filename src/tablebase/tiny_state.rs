use arrayvec::ArrayVec;

use crate::{
    board::{
        bit_array::BitArray,
        bit_board::BitBoard,
        player_color::PlayerColor,
        square::{self, Square},
    },
};
use std::fmt::Debug;
use std::hash::Hash;

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
            "\n{}\n Active: {:?} EpSquare {}",
            self.board.to_string(),
            self.active_color,
            self.ep_square.square_string()
        )
    }
}

type FlagList = ArrayVec<TinyState, 8>;

impl TinyState {
    pub fn gen_legal_gameflags(bb: &BitBoard) -> FlagList {
        let mut active_colors: ArrayVec<PlayerColor, 2> = ArrayVec::new();
        
        active_colors.push(PlayerColor::White);
        active_colors.push(PlayerColor::Black);
        
        
        // if !bb.has_symmetric_piece_count() {
        //     // active_colors.pop();
        // }

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
        tablebase::tiny_state::TinyState,
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
