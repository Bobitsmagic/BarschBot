use crate::{board::{piece_type::PieceType, player_color::PlayerColor, square::{self}}, game::game_state::GameState};

#[derive(Debug, Clone, Copy)]
pub struct WieselSettings {
    pawn_value: i32,
}

pub fn evaluation_function(gs: &GameState, wiesel_settings: WieselSettings) -> i32 {
    let board_state = &gs.board_state;

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
}