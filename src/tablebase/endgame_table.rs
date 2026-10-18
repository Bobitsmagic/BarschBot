use std::{collections::HashSet, sync::{Arc, Mutex}};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    board::{
        bit_array::BitArray, bit_array_lookup::{BISHOP_MOVE_TABLE, DIAGONAL_MOVES, KING_MOVES, ROWS}, bit_board::BitBoard, piece_board::PieceBoard, piece_type::{
            ColoredPieceType::{self, NonePiece},
            PieceType,
        }, player_color::PlayerColor, square::{self, A1, A2, A3, B1, B2, C1, C2, C3, D1, D2, D3, D4, Square, VALID_SQUARES},
    }, tablebase::{
        endgame_type::{self, EndGameType}, small_state::SmallState, tiny_state::TinyState,
    },
};

pub fn generate_king_boards() -> Vec<BitBoard> {
    //Should be 462 cases up to symmetry
    let mut ret = Vec::new();
    for s1 in VALID_SQUARES {
        let occupied = KING_MOVES[s1 as usize] | s1.bit_array();

        for s2 in (!occupied).iterate_squares() {
            let mut pt = BitBoard::empty();

            pt.add_piece(ColoredPieceType::WhiteKing, s1);
            pt.add_piece(ColoredPieceType::BlackKing, s2);

            ret.push(pt);
        }
    }

    return ret;
}

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
    set.extend(generate_king_boards_sym().into_iter().map(|board| 
    (TinyState { board, active_color: PlayerColor::White, ep_square: square::NONE}).find_representative().board));

    return set.into_iter().collect();

    todo!();
    //Should be 462 cases up to symmetry
    let mut list = Vec::new();

    let main_diag = DIAGONAL_MOVES[A1 as usize] | 1;

    for s1 in [A1, B2, C3, D4] {
        for s2 in (main_diag & !KING_MOVES[s1 as usize]).iterate_squares() {
            if s1 == s2 || s2.map_to_triangle() < s1 {
                continue;
            }

            let mut bb = BitBoard::empty();
            bb.add_piece(ColoredPieceType::WhiteKing, s1);
            bb.add_piece(ColoredPieceType::BlackKing, s2);
            list.push(bb);
        }

        for s2 in (!KING_MOVES[s1 as usize]).iterate_squares() {
            if s2.file() > s2.rank() && !s2.map_to_triangle() < s1  {
                let mut bb = BitBoard::empty();
                bb.add_piece(ColoredPieceType::WhiteKing, s1);
                bb.add_piece(ColoredPieceType::BlackKing, s2);
                list.push(bb);
            }
        }
    }

    for s1 in [B1, C1, D1, C2, D2, D3] {
        for s2 in (!KING_MOVES[s1 as usize]).iterate_squares() {
            if s1 == s2  || s2.map_to_triangle() < s1  {
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

pub fn generate_game_states(
    piece_list: &EndGameType,
    mut king_board: BitBoard,
    set: &mut HashSet<SmallState>,
) {
    fn backtrack_boards(
        piece_index: usize,
        mut min_pos: i8,
        prev_piece_type: ColoredPieceType,
        board: &mut BitBoard,
        piece_list: &Vec<ColoredPieceType>,
        ret_list: &mut HashSet<SmallState>,
    ) {
        if piece_index == piece_list.len() {
            for ss in SmallState::gen_legal_gameflags(board) {
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
}

pub fn generate_game_states_sym_king_boards(
    piece_list: &EndGameType,
    mut king_board: BitBoard,
    set: &mut HashSet<SmallState>,
) {
    fn backtrack_boards(
        piece_index: usize,
        mut min_pos: i8,
        prev_piece_type: ColoredPieceType,
        board: &mut BitBoard,
        piece_list: &Vec<ColoredPieceType>,
        ret_list: &mut HashSet<SmallState>,
    ) {
        if piece_index == piece_list.len() {
            for ss in SmallState::gen_legal_gameflags(board) {
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

pub fn count_boards_sym(pc: usize) -> HashSet<SmallState> {
    let kl = generate_king_boards_sym();
    let pt_list = endgame_type::generate_piece_lists_sym(pc);

    let mut sum = 0;
    for king_board in &kl {
        for et in &pt_list {
            let mut set = HashSet::new();
            generate_game_states_sym_king_boards(et, king_board.clone(), &mut set);
            sum += set.len();
        }
    }
    println!("Sum: {}", sum);

    return HashSet::new();
}

pub fn count_boards(pc: usize) -> HashSet<SmallState> {
    let kl = generate_king_boards();
    let pt_list = endgame_type::generate_piece_lists_sym(pc);

    let mut sum = 0;
    for et in &pt_list {
        et.print();
        let mut set = HashSet::new();
        for king_board in &kl {
            generate_game_states(et, king_board.clone(), &mut set);
        }
        sum += set.len();
    }
    println!("Sum: {}", sum);

    return HashSet::new();
}

pub fn count_boards_sym_tiny(pc: usize) -> Vec<TinyState> {
    let kl = generate_king_boards_sym();
    let kl_color = generate_king_boards_sym_color();
    
    let pt_list = endgame_type::generate_piece_lists_sym(pc);

    println!("Type list: {}", pt_list.len());

    let sum = Arc::new(Mutex::new(0));
    let max = Arc::new(Mutex::new(0));
    let count = Arc::new(Mutex::new(0));

    rayon::ThreadPoolBuilder::new()
        .num_threads(6)
        .build_global()
        .unwrap();

    pt_list.par_iter().for_each(|et| {
        et.print();
        let mut set = HashSet::new();
        for king_board in if et.is_symmetric() { &kl_color } else { &kl } {    
            set.clear();
            generate_game_states_sym_king_boards_tiny(et, king_board.clone(), &mut set);
            
            *sum.lock().unwrap() += set.len();
            let mut lock = max.lock().unwrap();
            *lock = set.len().max(*lock)
        }
        *count.lock().unwrap() += 1;
        
        println!("Count: {} Sum: {} max {}", *count.lock().unwrap(), *sum.lock().unwrap(), *max.lock().unwrap());
    });
    
    println!("Sum: {}", *sum.lock().unwrap());
    println!("Max: {}", *max.lock().unwrap());

    return Vec::new();
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