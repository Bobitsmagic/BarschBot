use std::collections::HashSet;

use crate::{
    board::{
        bit_array::BitArray,
        bit_array_lookup::{DIAGONAL_MOVES, KING_MOVES, ROWS},
        bit_board::BitBoard,
        piece_type::{ColoredPieceType, PieceType},
        player_color::PlayerColor,
        square::{self, Square, A1, B1, B2, C1, C2, C3, D1, D2, D3, D4},
    },
    tablebase::endgame::{
        endgame_type::{self, EndGameType},
        tiny_state::TinyState,
    },
};

pub fn generate_king_boards_sym() -> Vec<BitBoard> {
    //Should be 462 cases up to symmetry
    let mut list = Vec::new();

    let main_diag = DIAGONAL_MOVES[A1 as usize] | 1;

    for s1 in [A1, B2, C3, D4] {
        for s2 in (main_diag & !KING_MOVES[s1 as usize]).iterate_squares() {
            if s1 == s2 {
                continue;
            }

            let mut bb = BitBoard::empty();
            bb.add_piece(ColoredPieceType::WhiteKing, s1);
            bb.add_piece(ColoredPieceType::BlackKing, s2);
            list.push(bb);
        }

        for s2 in (!KING_MOVES[s1 as usize]).iterate_squares() {
            if s2.file() > s2.rank() {
                let mut bb = BitBoard::empty();
                bb.add_piece(ColoredPieceType::WhiteKing, s1);
                bb.add_piece(ColoredPieceType::BlackKing, s2);
                list.push(bb);
            }
        }
    }

    for s1 in [B1, C1, D1, C2, D2, D3] {
        for s2 in (!KING_MOVES[s1 as usize]).iterate_squares() {
            if s1 == s2 {
                continue;
            }

            let mut bb = BitBoard::empty();
            bb.add_piece(ColoredPieceType::WhiteKing, s1);
            bb.add_piece(ColoredPieceType::BlackKing, s2);
            list.push(bb);
        }
    }

    return list;
}

pub fn generate_king_boards_sym_color() -> Vec<BitBoard> {
    let mut set: HashSet<BitBoard> = HashSet::new();
    set.extend(generate_king_boards_sym().into_iter().map(|board| {
        (TinyState {
            board,
            active_color: PlayerColor::White,
            ep_square: square::NONE,
        })
        .find_representative()
        .board
    }));

    return set.into_iter().collect();
}

pub fn generate_game_states_sym_king_boards_tiny(
    piece_list: &EndGameType,
    mut king_board: BitBoard,
    set: &mut HashSet<TinyState>,
) {
    fn backtrack_boards(
        piece_index: usize,
        mut min_pos: i8,
        prev_piece_type: ColoredPieceType,
        board: &mut BitBoard,
        piece_list: &Vec<ColoredPieceType>,
        ret_list: &mut HashSet<TinyState>,
    ) {
        if piece_index == piece_list.len() {
            for ss in TinyState::gen_legal_gameflags(board) {
                ret_list.insert(ss.find_representative());
            }

            return;
        }

        let pt = piece_list[piece_index];
        if pt != prev_piece_type {
            min_pos = 0;
        }
        if min_pos >= 64 {
            return;
        }

        let blocker = if pt.piece_type() == PieceType::Pawn {
            ROWS[0] | ROWS[7]
        } else {
            0
        };

        let occ = (board.white_piece | board.black_piece) | ((1 << min_pos) - 1) | blocker;
        for square in (!occ).iterate_squares() {
            board.add_piece(pt, square);

            backtrack_boards(piece_index + 1, square + 1, pt, board, piece_list, ret_list);

            board.remove_piece(pt, square);
        }
    }

    backtrack_boards(
        0,
        0,
        ColoredPieceType::NonePiece,
        &mut king_board,
        &piece_list.get_piece_list(),
        set,
    );

    if piece_list.has_pawns() {
        backtrack_boards(
            0,
            0,
            ColoredPieceType::NonePiece,
            &mut king_board.flip_y(),
            &piece_list.get_piece_list(),
            set,
        );

        backtrack_boards(
            0,
            0,
            ColoredPieceType::NonePiece,
            &mut king_board.transpose(),
            &piece_list.get_piece_list(),
            set,
        );

        backtrack_boards(
            0,
            0,
            ColoredPieceType::NonePiece,
            &mut king_board.transpose().flip_y(),
            &piece_list.get_piece_list(),
            set,
        );
    }
}

pub fn gen_all_stats(endgame: &EndGameType, king_list: &Vec<BitBoard>) -> Vec<TinyState> {
    if endgame.is_symmetric() {
        assert_eq!(king_list.len(), 246)
    } else {
        assert_eq!(king_list.len(), 462)
    }

    let mut list = Vec::new();
    let mut set = HashSet::new();
    for king_board in king_list {
        assert!(set.len() == 0);
        generate_game_states_sym_king_boards_tiny(endgame, king_board.clone(), &mut set);

        list.extend(set.drain());
    }

    return list;
}

pub fn gen_all_endgames(piece_count: usize) -> Vec<Vec<TinyState>> {
    let king_list = generate_king_boards_sym();
    let king_list_color = generate_king_boards_sym_color();

    let mut endgame_list = endgame_type::generate_piece_lists_sym(piece_count);

    endgame_list.sort();

    let mut ret_list = Vec::new();
    for egt in &endgame_list {
        let kl = if egt.is_symmetric() {
            &king_list_color
        } else {
            &king_list
        };

        let list = gen_all_stats(egt, kl);
        print!("{} ", list.len());
        egt.to_string();

        ret_list.push(list);
    }

    return ret_list;
}

pub fn check_hashes(piece_count: usize) {
    let all_boards = gen_all_endgames(piece_count);
    let count = all_boards.len();
    println!("Count: {}", count);

    let mut set = HashSet::new();

    for ts in all_boards.into_iter().flatten() {
        if !set.insert(ts.get_hash_value()) {
            println!("Oh no");
        }
    }

    println!("Set count: {}", set.len());
}
