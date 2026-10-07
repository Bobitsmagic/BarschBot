use arrayvec::ArrayVec;
use rand::Rng;

use crate::{
    board::{
        bit_array::BitArray, bit_array_lookup::{KING_MOVES, KNIGHT_MOVES, PAWN_MOVES_BLACK, PAWN_MOVES_WHITE}, bit_board::BitBoard, dynamic_state::DynamicState, piece_type::{
            ColoredPieceType::{self, *}, PieceType,
        }, player_color::PlayerColor::{self, Black, White}, square::*,
    }, evaluation::search_functions::get_random_pos, game::{board_state::BoardState, game_result::GameResult, game_state::GameState}, moves::{chess_move::ChessMove, kogge_gen::{self, DIAGONAL_FILL_FUNCTIONS, ORTHOGONAL_FILL_FUNCTIONS}, move_gen, slider_gen},
};

fn get_smallest_attacker(square: i8, side: PlayerColor, bb: &BitBoard) -> Option<(PieceType, i8)> {
    let occupied = bb.white_piece | bb.black_piece;

    let pc = match side {
        White => bb.white_piece,
        Black => bb.black_piece,
    };

    let pawn_attacks = pc
        & bb.pawn
        & match side {
            White => PAWN_MOVES_BLACK[square as usize],
            Black => PAWN_MOVES_WHITE[square as usize],
        };
    if pawn_attacks != 0 {
        return Some((PieceType::Pawn, pawn_attacks.lowest_square_index() as i8));
    }

    let knight_attacks = pc & bb.knight & KNIGHT_MOVES[square as usize];
    if knight_attacks != 0 {
        return Some((
            PieceType::Knight,
            knight_attacks.lowest_square_index() as i8,
        ));
    }

    let diagonal_attacker = pc
        & bb.diagonal_slider
        & slider_gen::gen_bishop_moves_kogge_occ(square.bit_array(), occupied);

    let bishop_attacker = diagonal_attacker & !bb.orthogonal_slider;
    if bishop_attacker != 0 {
        return Some((
            PieceType::Bishop,
            bishop_attacker.lowest_square_index() as i8,
        ));
    }

    let orthogonal_attacker =
        pc & bb.orthogonal_slider & slider_gen::gen_rook_moves_kindergarten(square, occupied);

    let rook_attacker = orthogonal_attacker & !bb.diagonal_slider;
    if rook_attacker != 0 {
        return Some((PieceType::Rook, rook_attacker.lowest_square_index() as i8));
    }

    let queen_attacker = diagonal_attacker | orthogonal_attacker;
    if queen_attacker != 0 {
        return Some((PieceType::Queen, queen_attacker.lowest_square_index() as i8));
    }

    let king_attacks = pc & bb.king & KING_MOVES[square as usize];
    if king_attacks != 0 {
        return Some((PieceType::King, king_attacks.lowest_square_index() as i8));
    }

    return None;
}

const PIECE_VALUE: [i32; 7] = [10, 28, 32, 50, 90, 1000, 10];
fn recursive_see(square: i8, side: PlayerColor, bb: &mut BoardState) -> i32 {
    let mut value = 0;
    if let Some((piece, start)) = get_smallest_attacker(square, side, &bb.bit_board) {
        let captured = bb.piece_board[square];
        let cm = ChessMove::new(start, square, piece.colored(side), captured);
        let piece_value = PIECE_VALUE[captured.piece_type() as usize];

        // println!("Capture: {} -> {}", cm.to_string(), piece_value);

        bb.make_move(cm);

        value = 0.max(piece_value - recursive_see(square, !side, bb));

        bb.undo_move(cm);
    }

    return value;
}

pub fn see_capture(cm: ChessMove, bb: &mut BoardState) -> i32 {
    debug_assert!(cm.is_capture());

    bb.make_move(cm);
    let value = PIECE_VALUE[cm.captured_piece.piece_type() as usize]
        - recursive_see(cm.end, !cm.move_piece.color(), bb);
    bb.undo_move(cm);

    return value;
}

pub fn see_capture_iterative(cm: ChessMove, bb: &mut BoardState) -> i32 {
    debug_assert!(cm.is_capture());

    bb.make_move(cm);
    let value = PIECE_VALUE[cm.captured_piece.piece_type() as usize]
        - iterative_see(cm.end, !cm.move_piece.color(), bb);
    bb.undo_move(cm);

    return value;
}


fn iterative_see(square: i8, mut side: PlayerColor, bs: &BoardState) -> i32 {
    let bb = &bs.bit_board;
    
    let mut slider_lists = ArrayVec::<ArrayVec::<ColoredPieceType,10>, 10>::new();
    
    let sq_field = square.bit_array();
    let orth_free = !((bb.black_piece | bb.white_piece) & !bb.orthogonal_slider) | sq_field;
    for (f, rev) in ORTHOGONAL_FILL_FUNCTIONS.iter().zip([true, false, false, true]) {
        let slider = f(sq_field, orth_free) & bb.orthogonal_slider & !sq_field;
        
        if slider != 0 {
            let mut list = ArrayVec::<ColoredPieceType, 10>::new();
            
            for a_sq in slider.iterate_squares() {
                list.push(bs.piece_board[a_sq]);
            }
            
            if rev {
                list.reverse();
            }
            slider_lists.push(list);
        }
    }
    
    let pawn_attacks = bb.pawn & ((PAWN_MOVES_BLACK[square as usize] & bb.white_piece) | (PAWN_MOVES_WHITE[square as usize] & bb.black_piece));

    let slider_and_close_pawns = bb.diagonal_slider | pawn_attacks;

    let diag_free = !((bb.black_piece | bb.white_piece) & !bb.diagonal_slider &!pawn_attacks);
    for (f, rev) in DIAGONAL_FILL_FUNCTIONS.iter().zip([true, true, false, false]) {
        let slider = f(sq_field, diag_free) & slider_and_close_pawns & !sq_field;
    
        if slider != 0 {
            let mut list = ArrayVec::<ColoredPieceType, 10>::new();
    
            for a_sq in slider.iterate_squares() {
                list.push(bs.piece_board[a_sq]);
            }
            
            if rev {
                list.reverse();
            }
            slider_lists.push(list);
        }
    }

    let king_attacks = KING_MOVES[square as usize] & bb.king;
    if king_attacks != 0 {
        let mut king_list = ArrayVec::<ColoredPieceType, 10>::new();
        for a_sq in king_attacks.iterate_squares() {
            king_list.push(bs.piece_board[a_sq]);
        }
        slider_lists.push(king_list);
    }
    
    let knight_attacks = bb.knight & KNIGHT_MOVES[square as usize];
    if knight_attacks & bb.white_piece != 0 {
        let mut knight_list = ArrayVec::<ColoredPieceType, 10>::new();
        for a_sq in (knight_attacks & bb.white_piece).iterate_squares() {
            knight_list.push(bs.piece_board[a_sq]);
        }
        slider_lists.push(knight_list);
    }
    if knight_attacks & bb.black_piece != 0 {
        let mut knight_list = ArrayVec::<ColoredPieceType, 10>::new();
        for a_sq in (knight_attacks & bb.black_piece).iterate_squares() {
            knight_list.push(bs.piece_board[a_sq]);
        }
        slider_lists.push(knight_list);
    }

    // for l in &slider_lists {
    //     println!("{:?}", l);
    // }

    let mut gain = ArrayVec::<i32, 32>::new();
    gain.push(PIECE_VALUE[bs.piece_board[square].piece_type() as usize]);
    while slider_lists.len() > 0 {
        let mut index = usize::MAX;
        let mut smallest_value = i32::MAX;
        
        // println!("Kek");
        for i in 0..slider_lists.len() {
            let cpt = slider_lists[i].last().unwrap();
            let val = PIECE_VALUE[cpt.piece_type() as usize];

            // println!("{:?} -> {}", cpt, val);
            if cpt.color() == side && val < smallest_value {
                index = i;
                smallest_value = val;

                // println!("new: {}", val);
            }
        }

        if index == usize::MAX {
            break;
        }

        let cpt = slider_lists[index].pop().unwrap();
        // dbg!(cpt);
        if slider_lists[index].len() == 0 {
            slider_lists.remove(index);
        }

        gain.push(smallest_value - gain[gain.len() - 1]);
        
        side = !side;
    }

    gain.push(0);

    for i in (1..gain.len()).rev() {
        gain[i - 1] = -((-gain[i - 1]).max(gain[i]))
    }
    
    return gain[0];
}

#[test]
fn test_recursive_see() {
    let mut gs = GameState::from_fen("1k1r4/1pp4p/p7/4p3/8/P5P1/1PP4P/2K1R3 w - - 0 1");
    assert_eq!(
        1000,
        see_capture(
            ChessMove::new(E2, E5, WhiteRook, BlackPawn),
            &mut gs.board_state
        )
    );

    let mut gs = GameState::from_fen("1k1r3q/1ppn3p/p4b2/4p3/8/P2N2P1/1PP1R1BP/2K1Q3 w - - 0 1");
    assert_eq!(
        -4000,
        see_capture(
            ChessMove::new(E2, E5, WhiteRook, BlackPawn),
            &mut gs.board_state
        )
    );
    assert_eq!(
        -1800,
        see_capture(
            ChessMove::new(D3, E5, WhiteKnight, BlackPawn),
            &mut gs.board_state
        )
    );

    let mut gs =
        GameState::from_fen("3rk2r/pppqp1bp/1n2bnp1/3p4/4P3/1BN2Q2/PPP1NPPP/2KR3R w k - 0 1");
    assert_eq!(
        0,
        see_capture(
            ChessMove::new(E4, D5, WhitePawn, BlackPawn),
            &mut gs.board_state
        )
    );
    assert_eq!(
        0,
        see_capture(
            ChessMove::new(C3, D5, WhiteKnight, BlackPawn),
            &mut gs.board_state
        )
    );
    assert_eq!(
        -400,
        see_capture(
            ChessMove::new(B3, D5, WhiteBishop, BlackPawn),
            &mut gs.board_state
        )
    );
    assert_eq!(
        -2200,
        see_capture(
            ChessMove::new(D1, D5, WhiteRook, BlackPawn),
            &mut gs.board_state
        )
    );


    // let mut gs =
    //     GameState::from_fen("3r2b1/2Nrnb2/1Nkqbn2/3p2rr/1NKRBN2/2NQNB2/3R2B1/3R3B w - - 0 1");
    // println!("Kek");
    // iterative_see(D5, White, &gs.board_state);
}

#[test]
fn test_fens() {
    const POS_COUNT: usize = 10000;
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(0);
    
    for _ in 0..POS_COUNT {
        let depth = rng.gen_range(50..100);

        let mut gs = loop {
            let gs = get_random_pos(depth, &mut rng);

            if matches!(gs.game_result(), GameResult::Undecided) {
                break gs;
            }
        };

        let moves = gs.gen_legal_moves();

        for m in moves {
            if !m.is_capture() {
                continue;
            }

            gs.board_state.piece_board.print();
            m.print();
            let v1 = see_capture(m, &mut gs.board_state);
            let v2 = see_capture_iterative(m, &mut gs.board_state);
            assert_eq!(v1, v2);
        }    
    }
}