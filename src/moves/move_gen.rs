use arrayvec::ArrayVec;

use crate::{board::{bit_array::BitArray, bit_array_lookup::{self, IN_BETWEEN_TABLE, ORTHOGONAL_MOVES, ROWS}, piece_board::PieceBoard, piece_type::{ColoredPieceType, PieceType}, player_color::PlayerColor, rank, square::Square}, game::{board_state::BoardState, game_flags::GameFlags}, moves::{check_pin_mask::CheckPinMask, slider_gen::{gen_bishop_moves_kogge, gen_bishop_moves_pext, gen_rook_moves_kogge, gen_rook_moves_pext}}};

use super::{chess_move::ChessMove, move_iterator::MoveIterator};

pub type MoveVector = ArrayVec<ChessMove, 200>;

pub fn count_moves(board_state: &BoardState, flags: &GameFlags) -> u32 {
    let mut count = 0;

    let board = &board_state.bit_board;

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

    //Pawns
    let pawns = board.pawn & allied;
    let dy = match moving_color {
        PlayerColor::White => 1,
        PlayerColor::Black => -1,
    };
    
    let double_push_destination_rank = match moving_color {
        PlayerColor::White => bit_array_lookup::ROWS[3],
        PlayerColor::Black => bit_array_lookup::ROWS[4],
    };
    
    //Captures
    let mut ep_bit = if flags.en_passant_square.is_valid_square() { flags.en_passant_square.bit_array() } else { 0 };

    ep_bit &= !pin_mask.diag.translate_vertical(dy); //The ep pawn can not be diagonally pinned

    let row_index = match moving_color {
        PlayerColor::White => 4,
        PlayerColor::Black => 3,
    };

    let king_square = (allied & board.king).lowest_square_index();
    //Horizontal ep pin
    let hz_attacker = ROWS[row_index] & board.orthogonal_slider & opponent & ORTHOGONAL_MOVES[king_square as usize];

    for attacker in hz_attacker.iterate_set_bits_indices() {
        let between = IN_BETWEEN_TABLE[attacker as usize][king_square as usize];
        if (between & occupied).count_ones() == 2 {
            let intersection = between & occupied;

            ep_bit &= !intersection.translate_vertical(dy);
        }
    }

    let pawn_targets = (opponent | ep_bit) & pin_mask.check | (ep_bit & pin_mask.check.translate_vertical(dy));

    let attack_pawns = pawns & !pin_mask.ortho; //Pawns that can attack

    //Pinned pawns
    let diagonal_pinned_pawns = attack_pawns & pin_mask.diag;

    //Pawns that are diagonally pinned need to stay on the pin mask
    let pawn_left_attacks = pawn_targets & diagonal_pinned_pawns.translate(-1, dy) & pin_mask.diag;
    count += count_pawn_moves(pawn_left_attacks);
    
    let pawn_right_attacks = pawn_targets & diagonal_pinned_pawns.translate(1, dy) & pin_mask.diag;
    count += count_pawn_moves(pawn_right_attacks);

    //Not pinned pawns
    let not_pinned_pawns = attack_pawns & !pin_mask.diag;

    let pawn_left_attacks = pawn_targets & not_pinned_pawns.translate(-1, dy);
    count += count_pawn_moves(pawn_left_attacks);

    let pawn_right_attacks = pawn_targets & not_pinned_pawns.translate(1, dy);
    count += count_pawn_moves(pawn_right_attacks);

    //Pushes
    let pushable_pawns = pawns & !pin_mask.diag;

    //Pinned pawns
    let orthogonal_pinned_pawns = pushable_pawns & pin_mask.ortho;

    let pinned_pushes = empty & orthogonal_pinned_pawns.translate_vertical(dy) & pin_mask.ortho & pin_mask.check;
    count += pinned_pushes.count_ones();

    //Not pinned pawns
    let not_pinned_pawns = pushable_pawns & !pin_mask.ortho ;
    let not_pinned_pushes = empty & not_pinned_pawns.translate_vertical(dy) & pin_mask.check;
    count += count_pawn_moves(not_pinned_pushes);

    //Double pushes
    let double_mask = pin_mask.check & double_push_destination_rank & empty;

    let pinned_double_pushes = double_mask & 
        ((pushable_pawns & pin_mask.ortho).translate_vertical(dy) & empty).translate_vertical(dy) & pin_mask.ortho; //Stay on pin

    count += pinned_double_pushes.count_ones();
    
    let not_pinned_double_pushes = double_mask & 
        ((pushable_pawns & !pin_mask.ortho).translate_vertical(dy) & empty).translate_vertical(dy);
    
    count += not_pinned_double_pushes.count_ones();

    //Knights
    let knights = board.knight & allied & !pin_mask.diag & !pin_mask.ortho;
    for square in knights.iterate_squares() {
        let moveset = bit_array_lookup::KNIGHT_MOVES[square as usize] & !allied & pin_mask.check;
        count += moveset.count_ones();
    }

    //Diagonal sliders
    let diagonal_sliders = board.diagonal_slider & allied & !pin_mask.ortho;

    let pinned_diagonal_sliders = diagonal_sliders & pin_mask.diag;
    for square in pinned_diagonal_sliders.iterate_squares() {
        let moveset = gen_bishop_moves_pext(square, occupied) & !allied & pin_mask.diag  & pin_mask.check; //Stay on pin
        count += moveset.count_ones();
    }

    let not_pinned_diagonal_sliders = diagonal_sliders & !pin_mask.diag;
    for square in not_pinned_diagonal_sliders.iterate_squares() {
        let moveset = gen_bishop_moves_pext(square, occupied) & !allied & pin_mask.check;
        count += moveset.count_ones();
    }

    //Orthogonal sliders
    let orthogonal_sliders = board.orthogonal_slider & allied & !pin_mask.diag;

    let pinned_orthogonal_sliders = orthogonal_sliders & pin_mask.ortho;
    for square in pinned_orthogonal_sliders.iterate_squares() {
        let moveset = gen_rook_moves_pext(square, occupied) & !allied & pin_mask.ortho & pin_mask.check; //Stay on pin
        count += moveset.count_ones();
    }

    let not_pinned_orthogonal_sliders = orthogonal_sliders & !pin_mask.ortho;
    for square in not_pinned_orthogonal_sliders.iterate_squares() {
        let moveset = gen_rook_moves_pext(square, occupied) & !allied & pin_mask.check;
        count += moveset.count_ones();
    }

    //King moves
    let king_square = (board.king & allied).lowest_square_index() as i8;
    let king_moves = bit_array_lookup::KING_MOVES[king_square as usize] & !allied;

    //Castling
    const QUEEN_SIDE_BLOCKER: u64 = 14 ; //B1, C1, D1
    const KING_SIDE_BLOCKER: u64 = 96; //F1, G1

    let translation = match moving_color {
        PlayerColor::White => 0,
        PlayerColor::Black => 7,
    };

    let queen_side_blocker = QUEEN_SIDE_BLOCKER.translate_vertical(translation);
    let king_side_blocker = KING_SIDE_BLOCKER.translate_vertical(translation);

    const QUEEN_SIDE_SQUARES: u64 = 12; //C1, D1

    let queen_side_squares = QUEEN_SIDE_SQUARES.translate_vertical(translation);

    let (king_side, queen_side) = match moving_color {
        PlayerColor::White => (flags.white_king_side_castle, flags.white_queen_side_castle),
        PlayerColor::Black => (flags.black_king_side_castle, flags.black_queen_side_castle),
    };
    
    let attacked_squares = board_state.bit_board.attacked_bits_through_king(!moving_color);
    count += (king_moves & !attacked_squares).count_ones();

    //Not currently in check
    if pin_mask.check == u64::MAX { 
        if queen_side && (occupied & queen_side_blocker) == 0 {
            if (queen_side_squares & attacked_squares) == 0 {
                count += 1;
            }
        }
    
        if king_side && (occupied & king_side_blocker) == 0 {
            if (king_side_blocker & attacked_squares) == 0 {
                count += 1;
            }
        }
    }

    return count;

    fn count_pawn_moves(targets: u64) -> u32 {
        const LAST_RANK: u64 = 0xFF000000000000FF;
        return (targets & !LAST_RANK).count_ones() + (targets & LAST_RANK).count_ones() * 4;
    }
}

pub fn gen_legal_moves_iterator(board_state: &BoardState, flags: &GameFlags) -> MoveIterator {
    let mut moves = MoveIterator::new();

    let board = &board_state.bit_board;

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

    //Pawns
    let pawns = board.pawn & allied;
    let dy = match moving_color {
        PlayerColor::White => 1,
        PlayerColor::Black => -1,
    };
    
    let double_push_destination_rank = match moving_color {
        PlayerColor::White => bit_array_lookup::ROWS[3],
        PlayerColor::Black => bit_array_lookup::ROWS[4],
    };
    
    //Captures
    let mut ep_bit = if flags.en_passant_square.is_valid_square() { flags.en_passant_square.bit_array() } else { 0 };

    ep_bit &= !pin_mask.diag.translate_vertical(dy); //The ep pawn can not be diagonally pinned

    let row_index = match moving_color {
        PlayerColor::White => 4,
        PlayerColor::Black => 3,
    };

    let king_square = (allied & board.king).lowest_square_index();
    //Horizontal ep pin
    let hz_attacker = ROWS[row_index] & board.orthogonal_slider & opponent & ORTHOGONAL_MOVES[king_square as usize];

    for attacker in hz_attacker.iterate_set_bits_indices() {
        let between = IN_BETWEEN_TABLE[attacker as usize][king_square as usize];
        if (between & occupied).count_ones() == 2 {
            let intersection = between & occupied;

            ep_bit &= !intersection.translate_vertical(dy);
        }
    }

    let pawn_targets = (opponent | ep_bit) & pin_mask.check | (ep_bit & pin_mask.check.translate_vertical(dy));

    let attack_pawns = pawns & !pin_mask.ortho; //Pawns that can attack

    //Pinned pawns
    let diagonal_pinned_pawns = attack_pawns & pin_mask.diag;

    //Pawns that are diagonally pinned need to stay on the pin mask
    let pawn_left_attacks = pawn_targets & diagonal_pinned_pawns.translate(-1, dy) & pin_mask.diag;
    moves.add_pawn_left_capture(pawn_left_attacks);
    
    let pawn_right_attacks = pawn_targets & diagonal_pinned_pawns.translate(1, dy) & pin_mask.diag;
    moves.add_pawn_right_capture(pawn_right_attacks);

    //Not pinned pawns
    let not_pinned_pawns = attack_pawns & !pin_mask.diag;

    let pawn_left_attacks = pawn_targets & not_pinned_pawns.translate(-1, dy);
    moves.add_pawn_left_capture(pawn_left_attacks);

    let pawn_right_attacks = pawn_targets & not_pinned_pawns.translate(1, dy);
    moves.add_pawn_right_capture(pawn_right_attacks);

    //Pushes
    let pushable_pawns = pawns & !pin_mask.diag;

    //Pinned pawns
    let orthogonal_pinned_pawns = pushable_pawns & pin_mask.ortho;

    let pinned_pushes = empty & orthogonal_pinned_pawns.translate_vertical(dy) & pin_mask.ortho & pin_mask.check;
    moves.add_pawn_push(pinned_pushes);

    //Not pinned pawns
    let not_pinned_pawns = pushable_pawns & !pin_mask.ortho ;
    let not_pinned_pushes = empty & not_pinned_pawns.translate_vertical(dy) & pin_mask.check;
    moves.add_pawn_push(not_pinned_pushes);

    //Double pushes
    let double_mask = pin_mask.check & double_push_destination_rank & empty;

    let pinned_double_pushes = double_mask & 
        ((pushable_pawns & pin_mask.ortho).translate_vertical(dy) & empty).translate_vertical(dy) & pin_mask.ortho; //Stay on pin

    moves.add_double_pawn_push(pinned_double_pushes);
    
    let not_pinned_double_pushes = double_mask & 
        ((pushable_pawns & !pin_mask.ortho).translate_vertical(dy) & empty).translate_vertical(dy);
    moves.add_double_pawn_push(not_pinned_double_pushes);

    //Knights
    let knights = board.knight & allied & !pin_mask.diag & !pin_mask.ortho;
    for square in knights.iterate_squares() {
        let moveset = bit_array_lookup::KNIGHT_MOVES[square as usize] & !allied & pin_mask.check;
        moves.add_move(square, moveset);
    }

    //Diagonal sliders
    let diagonal_sliders = board.diagonal_slider & allied & !pin_mask.ortho;

    let pinned_diagonal_sliders = diagonal_sliders & pin_mask.diag;
    for square in pinned_diagonal_sliders.iterate_squares() {
        let moveset = gen_bishop_moves_pext(square, occupied) & !allied & pin_mask.diag  & pin_mask.check; //Stay on pin
        moves.add_move(square, moveset);
    }

    let not_pinned_diagonal_sliders = diagonal_sliders & !pin_mask.diag;
    for square in not_pinned_diagonal_sliders.iterate_squares() {
        let moveset = gen_bishop_moves_pext(square, occupied) & !allied & pin_mask.check;
        moves.add_move(square, moveset);
    }

    //Orthogonal sliders
    let orthogonal_sliders = board.orthogonal_slider & allied & !pin_mask.diag;

    let pinned_orthogonal_sliders = orthogonal_sliders & pin_mask.ortho;
    for square in pinned_orthogonal_sliders.iterate_squares() {
        let moveset = gen_rook_moves_pext(square, occupied) & !allied & pin_mask.ortho & pin_mask.check; //Stay on pin
        moves.add_move(square, moveset);
    }

    let not_pinned_orthogonal_sliders = orthogonal_sliders & !pin_mask.ortho;
    for square in not_pinned_orthogonal_sliders.iterate_squares() {
        let moveset = gen_rook_moves_pext(square, occupied) & !allied & pin_mask.check;
        moves.add_move(square, moveset);
    }

    //King moves
    let king_square = (board.king & allied).lowest_square_index() as i8;
    let king_moves = bit_array_lookup::KING_MOVES[king_square as usize] & !allied;

    //Castling
    const QUEEN_SIDE_BLOCKER: u64 = 14 ; //B1, C1, D1
    const KING_SIDE_BLOCKER: u64 = 96; //F1, G1

    let translation = match moving_color {
        PlayerColor::White => 0,
        PlayerColor::Black => 7,
    };

    let queen_side_blocker = QUEEN_SIDE_BLOCKER.translate_vertical(translation);
    let king_side_blocker = KING_SIDE_BLOCKER.translate_vertical(translation);

    const QUEEN_SIDE_SQUARES: u64 = 12; //C1, D1

    let queen_side_squares = QUEEN_SIDE_SQUARES.translate_vertical(translation);

    let (king_side, queen_side) = match moving_color {
        PlayerColor::White => (flags.white_king_side_castle, flags.white_queen_side_castle),
        PlayerColor::Black => (flags.black_king_side_castle, flags.black_queen_side_castle),
    };
    
    let attacked_squares = board_state.bit_board.attacked_bits_through_king(!moving_color);

    moves.add_move(king_square, king_moves & !attacked_squares);

    //Not currently in check
    if pin_mask.check == u64::MAX { 
        if queen_side && (occupied & queen_side_blocker) == 0 {
            if (queen_side_squares & attacked_squares) == 0 {
                moves.add_move(king_square, king_square.translate(-2, 0).bit_array());
            }
        }
    
        if king_side && (occupied & king_side_blocker) == 0 {
            if (king_side_blocker & attacked_squares) == 0 {
                moves.add_move(king_square, king_square.translate(2, 0).bit_array());
            }
        }
    }

    return moves;
}

pub fn gen_legal_moves(board_state: &BoardState, flags: &GameFlags) -> MoveVector {
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

    let pin_mask = CheckPinMask::pins_on(moving_color, board);

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
    let mut ep_bit = if flags.en_passant_square.is_valid_square() { flags.en_passant_square.bit_array() } else { 0 };

    ep_bit &= !pin_mask.diag.translate_vertical(dy); //The ep pawn can not be diagonally pinned


    let row_index = match moving_color {
        PlayerColor::White => 4,
        PlayerColor::Black => 3,
    };

    let king_square = (allied & board.king).lowest_square_index();
    //Horizontal ep pin
    let hz_attacker = ROWS[row_index] & board.orthogonal_slider & opponent & ORTHOGONAL_MOVES[king_square as usize];

    for attacker in hz_attacker.iterate_set_bits_indices() {
        let between = IN_BETWEEN_TABLE[attacker as usize][king_square as usize];
        if (between & occupied).count_ones() == 2 {
            let intersection = between & occupied;

            ep_bit &= !intersection.translate_vertical(dy);
        }
    }

    let pawn_targets = (opponent | ep_bit) & pin_mask.check | (ep_bit & pin_mask.check.translate_vertical(dy));

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

    let pinned_pushes = empty & orthogonal_pinned_pawns.translate_vertical(dy) & pin_mask.ortho & pin_mask.check;
    for target_square in pinned_pushes.iterate_squares() {
        let start_square = target_square.translate(0, -dy);

        add_pawn_move(&mut moves, start_square, target_square, pawn_pt, piece_board);
    }

    //Not pinned pawns
    let not_pinned_pawns = pushable_pawns & !pin_mask.ortho ;
    let not_pinned_pushes = empty & not_pinned_pawns.translate_vertical(dy) & pin_mask.check;
    for target_square in not_pinned_pushes.iterate_squares() {
        let start_square = target_square.translate(0, -dy);

        add_pawn_move(&mut moves, start_square, target_square, pawn_pt, piece_board);
    }

    //Double pushes
    let double_mask = pin_mask.check & double_push_rank & empty;

    let pinned_double_pushes = double_mask & 
        ((pushable_pawns & pin_mask.ortho).translate_vertical(dy) & empty).translate_vertical(dy) & pin_mask.ortho; //Stay on pin

    for target_square in pinned_double_pushes.iterate_squares() {
        let start_square = target_square.translate(0, -2 * dy);
        moves.push(ChessMove::new(start_square, target_square, pawn_pt, piece_board[target_square]));
    }

    let not_pinned_double_pushes = double_mask & 
        ((pushable_pawns & !pin_mask.ortho).translate_vertical(dy) & empty).translate_vertical(dy);
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
        let moveset = gen_bishop_moves_kogge(square.bit_array(), allied, opponent) & pin_mask.diag  & pin_mask.check; //Stay on pin
        // let moveset = gen_bishop_moves_pext(square, occupied) & !allied & pin_mask.diag  & pin_mask.check; //Stay on pin
        for target_square in moveset.iterate_squares() {
            moves.push(ChessMove::new(square, target_square, pt, piece_board[target_square]));
        }
    }

    let not_pinned_diagonal_sliders = diagonal_sliders & !pin_mask.diag;
    for square in not_pinned_diagonal_sliders.iterate_squares() {
        let pt = piece_board[square];
        //_kogge(square.bit_array(), allied, opponent)
        let moveset = gen_bishop_moves_kogge(square.bit_array(), allied, opponent) & pin_mask.check;
        // let moveset = gen_bishop_moves_pext(square, occupied) & !allied & pin_mask.check;
        for target_square in moveset.iterate_squares() {
            moves.push(ChessMove::new(square, target_square, pt, piece_board[target_square]));
        }
    }

    //Orthogonal sliders
    let orthogonal_sliders = board.orthogonal_slider & allied & !pin_mask.diag;

    let pinned_orthogonal_sliders = orthogonal_sliders & pin_mask.ortho;
    for square in pinned_orthogonal_sliders.iterate_squares() {
        let pt = piece_board[square];
        let moveset = gen_rook_moves_kogge(square.bit_array(), allied, opponent) & pin_mask.ortho & pin_mask.check; //Stay on pin
        // let moveset = gen_rook_moves_pext(square, occupied) & !allied & pin_mask.ortho & pin_mask.check; //Stay on pin
        for target_square in moveset.iterate_squares() {
            moves.push(ChessMove::new(square, target_square, pt, piece_board[target_square]));
        }
    }

    let not_pinned_orthogonal_sliders = orthogonal_sliders & !pin_mask.ortho;
    for square in not_pinned_orthogonal_sliders.iterate_squares() {
        let pt = piece_board[square];
        // let moveset = gen_rook_moves_pext(square, occupied) & !allied & pin_mask.check;
        let moveset = gen_rook_moves_kogge(square.bit_array(), allied, opponent) & pin_mask.check;
        for target_square in moveset.iterate_squares() {
            moves.push(ChessMove::new(square, target_square, pt, piece_board[target_square]));
        }
    }

    //King moves
    let king_square = (board.king & allied).lowest_square_index() as i8;
    let king_moves = bit_array_lookup::KING_MOVES[king_square as usize] & !allied;

    //Castling
    const QUEEN_SIDE_BLOCKER: u64 = 14 ; //B1, C1, D1
    const KING_SIDE_BLOCKER: u64 = 96; //F1, G1

    let translation = match moving_color {
        PlayerColor::White => 0,
        PlayerColor::Black => 7,
    };

    let queen_side_blocker = QUEEN_SIDE_BLOCKER.translate_vertical(translation);
    let king_side_blocker = KING_SIDE_BLOCKER.translate_vertical(translation);

    const QUEEN_SIDE_SQUARES: u64 = 12; //C1, D1

    let queen_side_squares = QUEEN_SIDE_SQUARES.translate_vertical(translation);

    let (king_side, queen_side) = match moving_color {
        PlayerColor::White => (flags.white_king_side_castle, flags.white_queen_side_castle),
        PlayerColor::Black => (flags.black_king_side_castle, flags.black_queen_side_castle),
    };
    
    let attacked_squares = board_state.bit_board.attacked_bits_through_king(!moving_color);

    let king_pt = PieceType::King.colored(moving_color);
    for s in (king_moves & !attacked_squares).iterate_squares() {
        moves.push(ChessMove::new(king_square, s, king_pt, piece_board[s]));
    }

    //Not currently in check
    if pin_mask.check == u64::MAX { 
        if queen_side && (occupied & queen_side_blocker) == 0 {
            if (queen_side_squares & attacked_squares) == 0 {
                moves.push(ChessMove::new(king_square, king_square.translate(-2, 0), king_pt, ColoredPieceType::None));
            }
        }
    
        if king_side && (occupied & king_side_blocker) == 0 {
            if (king_side_blocker & attacked_squares) == 0 {
                moves.push(ChessMove::new(king_square, king_square.translate(2, 0), king_pt, ColoredPieceType::None));
            }
        }
    }

    return moves;
}

fn add_pawn_move(list: &mut MoveVector, start_square: i8, target_square: i8, pt: ColoredPieceType, piece_board: &PieceBoard) {
    let captured_piece = piece_board[target_square];

    if target_square.rank() == rank::R1 || target_square.rank() == rank::R8 {
        list.push(ChessMove::new_pawn(start_square, target_square, pt, captured_piece, PieceType::Queen.colored(pt.color())));
        list.push(ChessMove::new_pawn(start_square, target_square, pt, captured_piece, PieceType::Rook.colored(pt.color())));
        list.push(ChessMove::new_pawn(start_square, target_square, pt, captured_piece, PieceType::Bishop.colored(pt.color())));
        list.push(ChessMove::new_pawn(start_square, target_square, pt, captured_piece, PieceType::Knight.colored(pt.color())));
    } else {
        list.push(ChessMove::new(start_square, target_square, pt, captured_piece));
    }

}