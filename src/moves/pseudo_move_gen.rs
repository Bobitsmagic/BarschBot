use arrayvec::ArrayVec;

use crate::{board::{bit_array::{self, BitArray}, bit_array_lookup, dynamic_state::DynamicState, piece_board::PieceBoard, piece_type::{ColoredPieceType, PieceType}, player_color::PlayerColor, square::{File, Rank, Square}}, game::{board_state::BoardState, game_flags::GameFlags}, moves::slider_gen::{gen_bishop_moves, gen_rook_moves}};
use super::{chess_move::ChessMove, move_gen::MoveVector};
pub fn gen_legal_moves_bitboard(board_state: &BoardState, game_flags: &GameFlags) -> MoveVector {
    let moves = gen_pseudo_legal_moves_bitboard(board_state, game_flags);
    return filter_legal_moves_bitboard(moves, board_state);
}
pub fn filter_legal_moves_bitboard(moves: MoveVector, board_state: &BoardState) -> MoveVector {
    let mut legal_moves = ArrayVec::new();
    for m in moves {
        let mut board_state = board_state.clone();
        
        board_state.make_move(m);
        if !board_state.is_in_check(m.move_piece.color()) {
            legal_moves.push(m);
        }        
    }
    return legal_moves;
}
pub fn gen_pseudo_legal_moves_bitboard(board_state: &BoardState, flags: &GameFlags) -> MoveVector {
    let mut moves = ArrayVec::new();
    let board = &board_state.bit_board;
    let piece_board = &board_state.piece_board;
    let occupied = board.white_piece | board.black_piece;
    let moving_color = flags.active_color;
    let empty = !occupied;
    let allied = match moving_color {
        PlayerColor::White => board.white_piece,
        PlayerColor::Black => board.black_piece,
    };
    let opponent = match moving_color {
        PlayerColor::White => board.black_piece,
        PlayerColor::Black => board.white_piece,
    };
    //Pawns
    let pawns = board.pawn & allied;
    let dy = match moving_color {
        PlayerColor::White => 1,
        PlayerColor::Black => -1,
    };
    let pawn_pt = PieceType::Pawn.colored(moving_color);
    let double_push_rank = match moving_color {
        PlayerColor::White => bit_array_lookup::ROWS[3],
        PlayerColor::Black => bit_array_lookup::ROWS[4],
    };
    
    //Captures
    let pawn_targets = opponent | (if flags.en_passant_square != Square::None { flags.en_passant_square.bit_array() } else { 0 });
    let pawn_left_attacks = pawn_targets & pawns.translate(-1, dy);
    for target_square in pawn_left_attacks.iterate_squares() {
        let start_square = target_square.translate(1, -dy);
        add_pawn_move(&mut moves, start_square, target_square, pawn_pt, piece_board);
    }
    
    let pawn_right_attacks = pawn_targets & pawns.translate(1, dy);
    for target_square in pawn_right_attacks.iterate_squares() {
        let start_square = target_square.translate(-1, -dy);
        add_pawn_move(&mut moves, start_square, target_square, pawn_pt, piece_board);
    }
    //Pushes
    let pawn_pushes = empty & pawns.translate(0, dy);
    for target_square in pawn_pushes.iterate_squares() {
        let start_square = target_square.translate(0, -dy);
        add_pawn_move(&mut moves, start_square, target_square, pawn_pt, piece_board);
    }
    let pawn_double_pushes = double_push_rank & empty & pawn_pushes.translate(0, dy);
    for target_square in pawn_double_pushes.iterate_squares() {
        let start_square = target_square.translate(0, -2 * dy);
        moves.push(ChessMove::new(start_square, target_square, pawn_pt, piece_board[target_square]));
    }
    //Knights
    let knights = board.knight & allied;
    let knight_pt = PieceType::Knight.colored(moving_color);
    for square in knights.iterate_squares() {
        let moveset = bit_array_lookup::KNIGHT_MOVES[square as usize] & !allied;
        for target_square in moveset.iterate_squares() {
            moves.push(ChessMove::new(square, target_square, knight_pt, piece_board[target_square]));
        }
    }
    //Diagonal sliders
    let diagonal_sliders = board.diagonal_slider & allied;
    for square in diagonal_sliders.iterate_squares() {
        let pt = piece_board[square];
        let moveset = gen_bishop_moves(square, allied, opponent);
        for target_square in moveset.iterate_squares() {
            moves.push(ChessMove::new(square, target_square, pt, piece_board[target_square]));
        }
    }
    //Orthogonal sliders
    let orthogonal_sliders = board.orthogonal_slider & allied;
    for square in orthogonal_sliders.iterate_squares() {
        let pt = piece_board[square];
        let moveset = gen_rook_moves(square, allied, opponent);
        for target_square in moveset.iterate_squares() {
            moves.push(ChessMove::new(square, target_square, pt, piece_board[target_square]));
        }
    }
    //King moves
    let king_square = (board.king & allied).to_square();
    let moveset = bit_array_lookup::KING_MOVES[king_square as usize] & !allied;
    for target_square in moveset.iterate_squares() {
        moves.push(ChessMove::new(king_square, target_square, PieceType::King.colored(moving_color), piece_board[target_square]));
    }
    //Castling
    const QUEEN_SIDE_SQUARES: u64 =  14 ; //B1, C1, D1
    const KING_SIDE_SQUARES: u64 = 96; //F1, G1
    let queen_side_blocker = match moving_color {
        PlayerColor::White => QUEEN_SIDE_SQUARES,
        PlayerColor::Black => QUEEN_SIDE_SQUARES.translate(0, 7),
    };
    let king_side_blocker = match moving_color {
        PlayerColor::White => KING_SIDE_SQUARES,
        PlayerColor::Black => KING_SIDE_SQUARES.translate(0, 7),
    };
    let (king_side, queen_side) = match moving_color {
        PlayerColor::White => (flags.white_king_side_castle, flags.white_queen_side_castle),
        PlayerColor::Black => (flags.black_king_side_castle, flags.black_queen_side_castle),
    };
    if !board_state.square_attacked(king_square, !moving_color) { 
        if queen_side && (occupied & queen_side_blocker) == 0 {
            if king_square.file() == File::A {
                println!("Alarm");
                board_state.piece_board.print();    
                println!("Qeen side {}", queen_side);
                flags.print();
            }
            if !board_state.square_attacked(king_square.left(), !moving_color) {
                moves.push(ChessMove::new(king_square, king_square.translate(-2, 0), PieceType::King.colored(moving_color), ColoredPieceType::None));
            }
        }
    
        if king_side && (occupied & king_side_blocker) == 0 {
            if !board_state.square_attacked(king_square.right(), !moving_color) {
                moves.push(ChessMove::new(king_square, king_square.translate(2, 0), PieceType::King.colored(moving_color), ColoredPieceType::None));
            }
        }
    }
    return moves;
}
fn add_pawn_move(list: &mut MoveVector, start_square: Square, target_square: Square, pt: ColoredPieceType, piece_board: &PieceBoard) {
    let captured_piece = piece_board[target_square];
    if target_square.rank() == Rank::R1 || target_square.rank() == Rank::R8 {
        list.push(ChessMove::new_pawn(start_square, target_square, pt, captured_piece, PieceType::Queen.colored(pt.color())));
        list.push(ChessMove::new_pawn(start_square, target_square, pt, captured_piece, PieceType::Rook.colored(pt.color())));
        list.push(ChessMove::new_pawn(start_square, target_square, pt, captured_piece, PieceType::Bishop.colored(pt.color())));
        list.push(ChessMove::new_pawn(start_square, target_square, pt, captured_piece, PieceType::Knight.colored(pt.color())));
    } else {
        list.push(ChessMove::new(start_square, target_square, pt, captured_piece));
    }
}