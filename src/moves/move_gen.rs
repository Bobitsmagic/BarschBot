use crate::{board::{bit_array::{self, BitArray}, bit_array_lookup, bit_board::BitBoard, color::PlayerColor, piece_board::PieceBoard, piece_type::{ColoredPieceType, PieceType}}, game::game_flags::GameFlags};

use super::chess_move::ChessMove;

pub fn generate_pseudo_legal_moves_bitboard(board: &BitBoard, piece_board: &PieceBoard, game_state: &GameFlags) -> Vec<ChessMove> {
    let mut moves = Vec::new();

    let occupied = board.white_piece | board.black_piece;
    let moving_color = game_state.active_color;

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
    let pawn_left_attacks = opponent & pawns.translate(-1, dy);
    for target_square in pawn_left_attacks.iterate_squares() {
        let start_square = target_square.translate(1, dy);
        moves.push(ChessMove::new(start_square, target_square, pawn_pt, piece_board[target_square]));

        debug_assert!(piece_board[target_square] != ColoredPieceType::None);
        debug_assert!(piece_board[target_square].color() != moving_color);
    }
    
    let pawn_right_attacks = opponent & pawns.translate(1, dy);
    for target_square in pawn_right_attacks.iterate_squares() {
        let start_square = target_square.translate(-1, dy);
        moves.push(ChessMove::new(start_square, target_square, pawn_pt, piece_board[target_square]));
    }

    //Pushes
    let pawn_pushes = empty & pawns.translate(0, dy);
    for target_square in pawn_pushes.iterate_squares() {
        let start_square = target_square.translate(0, -dy);
        moves.push(ChessMove::new(start_square, target_square, pawn_pt, piece_board[target_square]));
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
        let moveset = bit_array::gen_bishop_moves(square, allied, opponent);
        for target_square in moveset.iterate_squares() {
            moves.push(ChessMove::new(square, target_square, pt, piece_board[target_square]));
        }
    }

    //Orthogonal sliders
    let orthogonal_sliders = board.orthogonal_slider & allied;
    for square in orthogonal_sliders.iterate_squares() {
        let pt = piece_board[square];
        let moveset = bit_array::gen_rook_moves(square, allied, opponent);
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
        PlayerColor::White => (game_state.white_king_side_castle, game_state.white_queen_side_castle),
        PlayerColor::Black => (game_state.black_king_side_castle, game_state.black_queen_side_castle),
    };

    if queen_side && (occupied & queen_side_blocker).is_empty() {
        moves.push(ChessMove::new(king_square, king_square.translate(-2, 0), PieceType::King.colored(moving_color), ColoredPieceType::None));
    }

    if king_side && (occupied & king_side_blocker).is_empty() {
        moves.push(ChessMove::new(king_square, king_square.translate(2, 0), PieceType::King.colored(moving_color), ColoredPieceType::None));
    }

    return moves;
}