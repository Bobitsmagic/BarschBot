use arrayvec::ArrayVec;

use crate::{
    board::{
        bit_array::BitArray,
        bit_board::BitBoard,
        dynamic_state::DynamicState,
        piece_board::PieceBoard,
        piece_type::ColoredPieceType,
        player_color::PlayerColor,
        square::{self, Square},
        zobrist_hash::ZobristHash,
    },
    game::{
        board_state::BoardState,
        game_flags::{self, GameFlags},
    },
};

use std::fmt::Debug;

use std::hash::{Hash, Hasher};

#[derive(Clone, PartialEq, Eq)]
pub struct SmallState {
    pub board_state: BoardState,
    pub game_flags: GameFlags,
    pub zobrist_hash: ZobristHash,
}

impl Debug for SmallState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n{}\nHash: {:0x}\n Flags: {:?}",
            self.board_state
                .piece_board
                .get_perspective(PlayerColor::White),
            self.zobrist_hash.hash,
            self.game_flags
        )
    }
}

impl Hash for SmallState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.zobrist_hash.hash.hash(state);
    }
}

impl SmallState {
    pub fn from_state_flags(bs: BoardState, flags: GameFlags) -> SmallState {
        let hash = ZobristHash::from_position(&bs.piece_board, flags);
        SmallState {
            board_state: bs,
            game_flags: flags,
            zobrist_hash: hash,
        }
    }

    pub fn gen_legal_gameflags(bb: &BitBoard) -> Vec<SmallState> {
        let pb = PieceBoard::from_bit_board(bb);
        let bs = BoardState {
            piece_board: pb,
            bit_board: bb.clone(),
        };

        let mut ret = Vec::with_capacity(4);

        let mut active_colors = vec![PlayerColor::White, PlayerColor::Black];
        active_colors.retain(|x| !bs.is_in_check(!*x));

        for active in active_colors {
            ret.push(SmallState::from_state_flags(
                bs.clone(),
                GameFlags {
                    active_color: active,
                    ..game_flags::EMPTY_FLAGS
                },
            ));

            for ep in bs.bit_board.get_valid_ep_squares(active).iterate_squares() {
                ret.push(SmallState::from_state_flags(
                    bs.clone(),
                    GameFlags {
                        active_color: active,
                        en_passant_square: ep,
                        ..game_flags::EMPTY_FLAGS
                    },
                ));
            }
        }

        return ret;
    }

    pub fn get_piece_list(&self) -> Vec<ColoredPieceType> {
        let bb = &self.board_state.bit_board;
        let occ = (bb.black_piece | bb.white_piece) & !bb.king;

        let mut list = Vec::with_capacity(occ.count_ones() as usize);
        for s in occ.iterate_squares() {
            list.push(self.board_state.piece_board[s]);
        }
        return list;
    }

    pub fn flip_x(&self) -> SmallState {
        let mut ret = SmallState::empty();

        let occ = self.board_state.bit_board.black_piece | self.board_state.bit_board.white_piece;

        for s in occ.iterate_squares() {
            ret.add_piece(self.board_state.piece_board[s], s.flip_x());
        }

        ret.game_flags = self.game_flags;
        if self.game_flags.en_passant_square != square::NONE {
            ret.game_flags.en_passant_square = self.game_flags.en_passant_square.flip_x();
        }

        ret.zobrist_hash.toggle_flags(ret.game_flags);

        return ret;
    }

    pub fn flip_y(&self) -> SmallState {
        let mut ret = SmallState::empty();

        let occ = self.board_state.bit_board.black_piece | self.board_state.bit_board.white_piece;

        for s in occ.iterate_squares() {
            ret.add_piece(self.board_state.piece_board[s], s.flip_y());
        }

        if self.game_flags.en_passant_square != square::NONE {
            panic!("Flipped along x altough there was a EP square")
        }

        ret.game_flags = self.game_flags;
        ret.zobrist_hash.toggle_flags(ret.game_flags);

        return ret;
    }

    pub fn transpose(&self) -> SmallState {
        let mut ret = SmallState::empty();

        let occ = self.board_state.bit_board.black_piece | self.board_state.bit_board.white_piece;

        for s in occ.iterate_squares() {
            ret.add_piece(self.board_state.piece_board[s], s.transpose());
        }

        if self.game_flags.en_passant_square != square::NONE {
            panic!("Transpose altough there was a EP square")
        }
        ret.game_flags = self.game_flags;
        ret.zobrist_hash.toggle_flags(ret.game_flags);

        return ret;
    }

    pub fn flip_color(&self) -> SmallState {
        let mut ret = SmallState::empty();

        let occ = self.board_state.bit_board.black_piece | self.board_state.bit_board.white_piece;

        for s in occ.iterate_squares() {
            ret.add_piece(self.board_state.piece_board[s].opposite(), s.flip_y());
        }

        ret.game_flags = self.game_flags;
        ret.game_flags.active_color = !self.game_flags.active_color;
        if self.game_flags.en_passant_square != square::NONE {
            ret.game_flags.en_passant_square = self.game_flags.en_passant_square.flip_y();
        }
        ret.zobrist_hash.toggle_flags(ret.game_flags);

        return ret;
    }

    pub fn print(&self) {
        self.board_state.piece_board.print();
        println!("Active: {:?}", self.game_flags.active_color);
        println!("Hash: {:0x}", self.zobrist_hash.hash)
    }

    pub fn find_representative(&self) -> SmallState {
        let mut all_boards: ArrayVec<SmallState, 16> = ArrayVec::new();

        all_boards.push(self.clone());
        all_boards.push(self.flip_x());

        for i in 0..all_boards.len() {
            all_boards.push(all_boards[i].flip_color());
        }

        if self.board_state.bit_board.pawn == 0 {
            for i in 0..all_boards.len() {
                all_boards.push(all_boards[i].flip_y());
            }

            for i in 0..all_boards.len() {
                all_boards.push(all_boards[i].transpose());
            }
        }

        return all_boards
            .into_iter()
            .min_by(|x, y| x.zobrist_hash.hash.cmp(&y.zobrist_hash.hash))
            .unwrap();
    }
}

impl DynamicState for SmallState {
    fn empty() -> Self {
        return SmallState {
            board_state: BoardState::empty(),
            game_flags: game_flags::EMPTY_FLAGS,
            zobrist_hash: ZobristHash::empty(),
        };
    }

    fn add_piece(&mut self, pt: ColoredPieceType, s: i8) {
        self.board_state.add_piece(pt, s);
        self.zobrist_hash.add_piece(pt, s);
    }

    fn remove_piece(&mut self, pt: ColoredPieceType, s: i8) {
        self.board_state.remove_piece(pt, s);
        self.zobrist_hash.remove_piece(pt, s);
    }
}

#[cfg(test)]
mod small_vec_test {
    use crate::{
        board::{
            bit_array::BitArray,
            dynamic_state::DynamicState,
            piece_type::ColoredPieceType,
            player_color::{PlayerColor, ALL_COLORS},
            square::*,
        },
        tablebase::small_state::SmallState,
    };

    #[test]
    fn test_identity() {
        for active in ALL_COLORS {
            for s1 in VALID_SQUARES {
                for s2 in VALID_SQUARES {
                    if s1 == s2 {
                        continue;
                    }

                    let mut sm = SmallState::empty();
                    sm.add_piece(ColoredPieceType::WhiteKing, s1);
                    sm.add_piece(ColoredPieceType::BlackKing, s2);

                    sm.game_flags.active_color = active;
                    sm.zobrist_hash.toggle_flags(sm.game_flags);

                    assert_eq!(sm, sm.flip_x().flip_x());
                    assert_eq!(sm, sm.flip_y().flip_y());
                    assert_eq!(sm, sm.transpose().transpose());
                    assert_eq!(sm, sm.flip_color().flip_color());

                    assert_eq!(sm, sm.flip_x().flip_y().flip_x().flip_y());
                }
            }
        }
    }

    #[test]
    fn test_static() {
        let mut b1 = SmallState::empty();
        let mut b2 = SmallState::empty();

        assert_eq!(b1.find_representative(), b2.find_representative());

        b1.add_piece(ColoredPieceType::WhiteKing, A1);

        assert_ne!(b1.find_representative(), b2.find_representative());

        b2.add_piece(ColoredPieceType::WhiteKing, H1);

        assert_eq!(b1.find_representative(), b2.find_representative());
    }

    #[test]
    fn test_all_king_pos() {
        for s1 in VALID_SQUARES {
            let mut b1 = SmallState::empty();
            b1.add_piece(ColoredPieceType::WhiteKing, s1);
            b1.zobrist_hash.toggle_flags(b1.game_flags);

            let repre = b1.find_representative();

            for pt in [ColoredPieceType::WhiteKing, ColoredPieceType::BlackKing] {
                let mut b2 = SmallState::empty();
                b2.game_flags.active_color = pt.color();
                b2.zobrist_hash.toggle_flags(b2.game_flags);

                let list = vec![
                    s1,
                    s1.flip_x(),
                    s1.flip_y(),
                    s1.rotate_180(),
                    s1.transpose(),
                    s1.transpose().flip_x(),
                    s1.transpose().flip_y(),
                    s1.transpose().rotate_180(),
                ];

                let mut squares = 0_u64;
                for s in list {
                    squares |= 1 << s;
                }

                for s in squares.iterate_squares() {
                    b2.add_piece(pt, s);
                    assert_eq!(b2.find_representative(), repre);
                    b2.remove_piece(pt, s);
                }

                for s in (!squares).iterate_squares() {
                    b2.add_piece(pt, s);
                    assert_ne!(b2.find_representative(), repre);
                    b2.remove_piece(pt, s);
                }
            }
        }
    }
}
