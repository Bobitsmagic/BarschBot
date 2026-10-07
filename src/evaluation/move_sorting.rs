use crate::{evaluation::static_exchange, game::board_state::BoardState, moves::{chess_move::ChessMove, move_gen::MoveVector}};

pub type QuietMoveTable = [[i64; 64]; 64];

pub fn see_move_sorter(list: &mut MoveVector, board: &mut BoardState, prev_best: ChessMove, quiet_move_table: &QuietMoveTable) {
    const PIECE_VALUES: [i64; 7] = [10, 28, 32, 50, 90, 0, 0];
    list.sort_by_cached_key(|cm| {
        if *cm == prev_best {
            return i64::MIN;
        }

        let mut sum = if board.bit_board.attacks_king(cm.move_piece, cm.end) {
            1000
        } else {
            0
        };

        if cm.is_direct_capture() {
            sum += static_exchange::see_capture(*cm, board) as i64;
        }

        if cm.is_en_passant() {
            sum += 200;
        }

        sum *= 1000;

        sum += PIECE_VALUES[cm.promotion_piece.piece_type() as usize];

        sum *= 1_000_000_000;
        
        sum += quiet_move_table[cm.start as usize][cm.end as usize];

        return -sum;
    });
}

pub fn better_move_sorter(list: &mut MoveVector, board: &BoardState, prev_best: ChessMove) {
    const PIECE_VALUES: [i32; 7] = [10, 28, 32, 50, 90, 100, 0];

    list.sort_by_cached_key(|cm| {
        if *cm == prev_best {
            return i32::MIN;
        }

        let mut sum = if board.bit_board.attacks_king(cm.move_piece, cm.end) {
            500
        } else {
            0
        };
        // let mut sum = 0;

        if cm.is_direct_capture() {
            sum += PIECE_VALUES[cm.captured_piece.piece_type() as usize]
                - PIECE_VALUES[cm.move_piece.piece_type() as usize]
                + 200;
        }

        if cm.is_en_passant() {
            sum += 200;
        }

        sum *= 1000;

        sum += PIECE_VALUES[cm.promotion_piece.piece_type() as usize];

        // sum *= 1000;

        //println!("Move: {} sum: {}", cm.get_board_name(&board), sum);

        return -sum;
    });

    //board.print_local_moves(&list);
}

pub fn quiet_move_sorter(
    list: &mut MoveVector,
    board: &BoardState,
    prev_best: ChessMove,
    quiet_move_table: &QuietMoveTable,
) {
    const PIECE_VALUES: [i64; 7] = [10, 28, 32, 50, 90, 100, 0];

    list.sort_by_cached_key(|cm| {
        if *cm == prev_best {
            return i64::MIN;
        }

        let mut sum = if board.bit_board.attacks_king(cm.move_piece, cm.end) {
            500
        } else {
            0
        };

        if cm.is_direct_capture() {
            sum += PIECE_VALUES[cm.captured_piece.piece_type() as usize]
                - PIECE_VALUES[cm.move_piece.piece_type() as usize]
                + 200;
        }

        if cm.is_en_passant() {
            sum += 200;
        }

        sum *= 1000;

        sum += PIECE_VALUES[cm.promotion_piece.piece_type() as usize];

        sum *= 1_000_000_000;

        if quiet_move_table[cm.start as usize][cm.end as usize] > 1_000_000_000 {
            // println!("Overflow in quiet move table");
        }
        sum += quiet_move_table[cm.start as usize][cm.end as usize];

        return -sum;
    });
}

pub fn killer_move_sorter(
    list: &mut MoveVector,
    board: &BoardState,
    prev_best: ChessMove,
    quiet_move_table: &QuietMoveTable,
    _killer_move: ChessMove,
) {
    const PIECE_VALUES: [i64; 7] = [10, 28, 32, 50, 90, 100, 0];

    list.sort_by_cached_key(|cm| {
        if *cm == prev_best {
            return i64::MIN;
        }

        let mut sum = if board.bit_board.attacks_king(cm.move_piece, cm.end) {
            500
        } else {
            0
        };

        if cm.is_direct_capture() {
            sum += PIECE_VALUES[cm.captured_piece.piece_type() as usize]
                - PIECE_VALUES[cm.move_piece.piece_type() as usize]
                + 200;
        }

        if cm.is_en_passant() {
            sum += 200;
        }

        sum *= 1000;

        sum += PIECE_VALUES[cm.promotion_piece.piece_type() as usize];

        sum *= 1_000_000_000;

        // if *cm == killer_move {
        //     sum += 100_000;
        // }

        if quiet_move_table[cm.start as usize][cm.end as usize] > 1_000_000_000 {
            // println!("Overflow in quiet move table");
        }
        sum += quiet_move_table[cm.start as usize][cm.end as usize];

        return -sum;
    });
}