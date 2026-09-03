use arrayvec::ArrayVec;

use crate::{board::{bit_array::BitArray, bit_array_lookup::KING_MOVES, bit_board::BitBoard, piece_type::{ColoredPieceType::{self, WhiteKing}, PieceType}, player_color::PlayerColor, square::{self, Square}}, game::{board_state, game_state::GameState}, moves::move_gen::{self, MoveVector}};

#[derive(Debug, Clone, Copy)]
pub struct WieselSettings {
    pub pawn_value: i32,
    pub version: i8,
    pub piece_weight: [i32; 5],
}

pub fn evaluation_function(gs: &GameState, wiesel_settings: WieselSettings) -> i32 {
    let board_state = &gs.board_state;
    let bb = &board_state.bit_board;

    let eval = match wiesel_settings.version {
                1 => { piece_count(bb) }
                2 => {
                        let mut whitepieces = [0_u64; 5];
                        let mut blackpieces = [0_u64; 5];

                        piece_array(bb, &mut whitepieces, &mut blackpieces);
                        static_piece_weight(&whitepieces, &blackpieces, wiesel_settings.piece_weight)
                    }
                3 => {
                        let mut eval = 0;
                        let mut whitepieces = [0_u64; 5];
                        let mut blackpieces = [0_u64; 5];

                        piece_array(bb, &mut whitepieces, &mut blackpieces);
                        eval += static_piece_weight(&whitepieces, &blackpieces, wiesel_settings.piece_weight);

                        let (whitemoves, blackmoves) = move_gen::gen_eval_moves(board_state);
                        eval += absolute_field_control(bb, &whitemoves, &blackmoves, &(|sum, &x| sum + (x as i32)));
                        eval
                    }
                4 => {
                        
                    }
                _ => 0
    };
    return eval;
}

    //DEMO
    /* let board_state = &gs.board_state;

    //Contains 
    let bb = &board_state.bit_board;
    let white_pawns = bb.white_piece & bb.pawn;
    let black_pawns = bb.black_piece & bb.pawn;

    let count_difference = white_pawns.count_ones() as i32 - black_pawns.count_ones() as i32;

    let mut result = count_difference * wiesel_settings.pawn_value;    
    let pawn_on_e4 = board_state.piece_board[square::E4];

    if pawn_on_e4.is_pawn() {
        result += match pawn_on_e4.color() {
            PlayerColor::White => 1,
            PlayerColor::Black => -1
        }
    }

    return result;
    */

pub fn piece_count(bb: &BitBoard) -> i32 {
        return bb.white_piece.count_ones() as i32 - bb.black_piece.count_ones() as i32;
}

pub fn piece_array(bb: &BitBoard, whitepieces: &mut [u64; 5], blackpieces: &mut [u64; 5]) {
    //porns
    whitepieces[0] = bb.white_piece & bb.pawn;
    blackpieces[0] = bb.black_piece & bb.pawn;
            
    //knights
    whitepieces[1] = bb.white_piece & bb.knight;
    blackpieces[1] = bb.black_piece & bb.knight;
            
    //bishops
    let bishops = bb.diagonal_slider & !bb.orthogonal_slider;
    whitepieces[2] = bb.white_piece & bishops;
    blackpieces[2] = bb.black_piece & bishops;
            
    //rooks
    let rooks = bb.orthogonal_slider & !bb.diagonal_slider;
    whitepieces[3] = bb.white_piece & rooks;
    blackpieces[3] = bb.black_piece & rooks;
            
    //queens
    let queens = bb.orthogonal_slider & bb.diagonal_slider;
    whitepieces[4] = bb.white_piece & queens;
    blackpieces[4]= bb.black_piece & queens;
}

pub fn static_piece_weight(whitepieces: &[u64; 5], blackpieces: &[u64; 5], weights: [i32; 5]) -> i32 {
    let mut sum = 0;
    for i in 0..5 {
        sum += (whitepieces[i].count_ones() as i32 - blackpieces[i].count_ones() as i32) * weights[i];
    }
    return sum;
}

pub fn absolute_field_control(bb: &BitBoard, whitemoves: &MoveVector, blackmoves: &MoveVector, f: &dyn Fn(i32, &i8) -> i32) -> i32 {
    let mut control: [i8; 64] = [0_i8; 64];

    for m in whitemoves {
        control[m.end as usize] += 1
    }
    for m in blackmoves {
        control[m.end as usize] -= 1
    }

    return control.iter().fold(0, |sum, &x| f(sum, &x))
}