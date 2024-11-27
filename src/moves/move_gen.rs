use crate::{board::{bit_array::{self, BitArray}, bit_array_lookup::{self, IN_BETWEEN_TABLE, ORTHOGONAL_MOVES, ROWS}, piece_board::PieceBoard, piece_type::{ColoredPieceType, PieceType}, player_color::PlayerColor, square::{Rank, Square}}, game::{board_state::BoardState, game_flags::GameFlags}, moves::check_pin_mask::CheckPinMask};

use super::chess_move::ChessMove;

pub fn gen_legal_moves_bitboard(board_state: &BoardState, flags: &GameFlags) -> Vec<ChessMove> {
    let mut moves = Vec::new();

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

    let pin_mask = CheckPinMask::pins_on(moving_color, board);

    // pin_mask.check.print();

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
    let mut ep_bit = if flags.en_passant_square != Square::None { flags.en_passant_square.bit_array() } else { BitArray::empty() };

    ep_bit &= !pin_mask.diag.translate(0, dy); //The ep pawn can not be diagonally pinned


    let row_index = match moving_color {
        PlayerColor::White => 4,
        PlayerColor::Black => 3,
    };

    let king_square = (allied & board.king).to_square();
    //Horizontal ep pin
    let hz_attacker = ROWS[row_index] & board.orthogonal_slider & opponent & ORTHOGONAL_MOVES[king_square as usize];

    for attacker in hz_attacker.iterate_squares() {
        let between = IN_BETWEEN_TABLE[attacker as usize][king_square as usize];
        if (between & occupied).count_bits() == 2 {
            let intersection = between & occupied;

            ep_bit &= !intersection.translate(0, dy);
        }
    }

    let pawn_targets = (opponent | ep_bit) & pin_mask.check | (ep_bit & pin_mask.check.translate(0, dy));

    let attack_pawns = pawns & !pin_mask.ortho; //Pawns that can attack

    //Pinned pawns
    let diagonal_pinned_pawns = attack_pawns & pin_mask.diag;

    //Pawns that are diagonally pinned need to stay on the pin mask
    let pawn_left_attacks = pawn_targets & diagonal_pinned_pawns.translate(-1, dy) & pin_mask.diag;
    for target_square in pawn_left_attacks.iterate_squares() {
        let start_square = target_square.translate(1, -dy);
        add_pawn_move(&mut moves, start_square, target_square, pawn_pt, piece_board);
    }

    let pawn_right_attacks = pawn_targets & diagonal_pinned_pawns.translate(1, dy) & pin_mask.diag;
    for target_square in pawn_right_attacks.iterate_squares() {
        let start_square = target_square.translate(-1, -dy);
        add_pawn_move(&mut moves, start_square, target_square, pawn_pt, piece_board);
    }

    //Not pinned pawns
    let not_pinned_pawns = attack_pawns & !pin_mask.diag;

    let pawn_right_attacks = pawn_targets & not_pinned_pawns.translate(-1, dy);
    for target_square in pawn_right_attacks.iterate_squares() {
        let start_square = target_square.translate(1, -dy);
        add_pawn_move(&mut moves, start_square, target_square, pawn_pt, piece_board);
    }

    let pawn_right_attacks = pawn_targets & not_pinned_pawns.translate(1, dy);
    for target_square in pawn_right_attacks.iterate_squares() {
        let start_square = target_square.translate(-1, -dy);
        add_pawn_move(&mut moves, start_square, target_square, pawn_pt, piece_board);
    }

    //Pushes
    let pushable_pawns = pawns & !pin_mask.diag;

    //Pinned pawns
    let orthogonal_pinned_pawns = pushable_pawns & pin_mask.ortho;

    let pinned_pushes = empty & orthogonal_pinned_pawns.translate(0, dy) & pin_mask.ortho & pin_mask.check;
    for target_square in pinned_pushes.iterate_squares() {
        let start_square = target_square.translate(0, -dy);

        add_pawn_move(&mut moves, start_square, target_square, pawn_pt, piece_board);
    }

    //Not pinned pawns
    let not_pinned_pawns = pushable_pawns & !pin_mask.ortho ;
    let not_pinned_pushes = empty & not_pinned_pawns.translate(0, dy) & pin_mask.check;
    for target_square in not_pinned_pushes.iterate_squares() {
        let start_square = target_square.translate(0, -dy);

        add_pawn_move(&mut moves, start_square, target_square, pawn_pt, piece_board);
    }

    //Double pushes
    let double_mask = pin_mask.check & double_push_rank & empty;

    let pinned_double_pushes = double_mask & 
        ((pushable_pawns & pin_mask.ortho).translate(0, dy) & empty).translate(0, dy) & pin_mask.ortho; //Stay on pin

    for target_square in pinned_double_pushes.iterate_squares() {
        let start_square = target_square.translate(0, -2 * dy);
        moves.push(ChessMove::new(start_square, target_square, pawn_pt, piece_board[target_square]));
    }

    let not_pinned_double_pushes = double_mask & 
        ((pushable_pawns & !pin_mask.ortho).translate(0, dy) & empty).translate(0, dy);
    for target_square in not_pinned_double_pushes.iterate_squares() {
        let start_square = target_square.translate(0, -2 * dy);
        moves.push(ChessMove::new(start_square, target_square, pawn_pt, piece_board[target_square]));
    }

    //Knights
    let knights = board.knight & allied & !pin_mask.diag & !pin_mask.ortho;
    let knight_pt = PieceType::Knight.colored(moving_color);
    for square in knights.iterate_squares() {
        let moveset = bit_array_lookup::KNIGHT_MOVES[square as usize] & !allied & pin_mask.check;
        for target_square in moveset.iterate_squares() {
            moves.push(ChessMove::new(square, target_square, knight_pt, piece_board[target_square]));
        }
    }

    //Diagonal sliders
    let diagonal_sliders = board.diagonal_slider & allied & !pin_mask.ortho;

    let pinned_diagonal_sliders = diagonal_sliders & pin_mask.diag;
    for square in pinned_diagonal_sliders.iterate_squares() {
        let pt = piece_board[square];
        let moveset = bit_array::gen_bishop_moves(square, allied, opponent) & pin_mask.diag  & pin_mask.check; //Stay on pin
        for target_square in moveset.iterate_squares() {
            moves.push(ChessMove::new(square, target_square, pt, piece_board[target_square]));
        }
    }

    let not_pinned_diagonal_sliders = diagonal_sliders & !pin_mask.diag;
    for square in not_pinned_diagonal_sliders.iterate_squares() {
        let pt = piece_board[square];
        let moveset = bit_array::gen_bishop_moves(square, allied, opponent) & pin_mask.check;
        for target_square in moveset.iterate_squares() {
            moves.push(ChessMove::new(square, target_square, pt, piece_board[target_square]));
        }
    }

    //Orthogonal sliders
    let orthogonal_sliders = board.orthogonal_slider & allied & !pin_mask.diag;

    let pinned_orthogonal_sliders = orthogonal_sliders & pin_mask.ortho;
    for square in pinned_orthogonal_sliders.iterate_squares() {
        let pt = piece_board[square];
        let moveset = bit_array::gen_rook_moves(square, allied, opponent) & pin_mask.ortho & pin_mask.check; //Stay on pin
        for target_square in moveset.iterate_squares() {
            moves.push(ChessMove::new(square, target_square, pt, piece_board[target_square]));
        }
    }

    let not_pinned_orthogonal_sliders = orthogonal_sliders & !pin_mask.ortho;
    for square in not_pinned_orthogonal_sliders.iterate_squares() {
        let pt = piece_board[square];
        let moveset = bit_array::gen_rook_moves(square, allied, opponent) & pin_mask.check;
        for target_square in moveset.iterate_squares() {
            moves.push(ChessMove::new(square, target_square, pt, piece_board[target_square]));
        }
    }

    //King moves
    let king_square = (board.king & allied).to_square();
    let moveset = bit_array_lookup::KING_MOVES[king_square as usize] & !allied;
    for target_square in moveset.iterate_squares() {
        if !board_state.bit_board.square_is_attacked_through_king(target_square, !moving_color) {
            moves.push(ChessMove::new(king_square, target_square, PieceType::King.colored(moving_color), piece_board[target_square]));
        }
    }

    //Castling
    const QUEEN_SIDE_SQUARES: BitArray = BitArray { bits: 14 }; //B1, C1, D1
    const KING_SIDE_SQUARES: BitArray = BitArray { bits: 96 }; //F1, G1

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

    //Not currently in check
    if pin_mask.check.is_full() { 
        if queen_side && (occupied & queen_side_blocker).is_empty() {
            if !board_state.square_attacked(king_square.left(), !moving_color) && 
               !board_state.square_attacked(king_square.left().left(), !moving_color) {
                moves.push(ChessMove::new(king_square, king_square.translate(-2, 0), PieceType::King.colored(moving_color), ColoredPieceType::None));
            }
        }
    
        if king_side && (occupied & king_side_blocker).is_empty() {
            if !board_state.square_attacked(king_square.right(), !moving_color) &&
               !board_state.square_attacked(king_square.right().right(), !moving_color) {
                moves.push(ChessMove::new(king_square, king_square.translate(2, 0), PieceType::King.colored(moving_color), ColoredPieceType::None));
            }
        }
    }

    return moves;
}

fn add_pawn_move(list: &mut Vec<ChessMove>, start_square: Square, target_square: Square, pt: ColoredPieceType, piece_board: &PieceBoard) {
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