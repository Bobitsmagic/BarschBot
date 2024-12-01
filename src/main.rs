use std::ops::Shl;

use barschbot::{board::{bit_array::{self, BitArray}, piece_type::ColoredPieceType, square::{A2, A4}}, fen::fen_helper, game::game_state::GameState, moves::chess_move::ChessMove};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
fn main() {
    let mut game_state = GameState::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -");

    game_state.make_move(ChessMove::new(A2, A4, ColoredPieceType::WhitePawn, ColoredPieceType::None));
    
}

fn test_random_moves() {
    let mut game_state = GameState::start_position();

    let mut rng = ChaCha8Rng::seed_from_u64(0);
    loop {
        
        let moves = game_state.gen_legal_moves();
        
        if moves.len() == 0 {
            break;
        }
        
        game_state.board_state.piece_board.print();

        let m = moves[rng.gen_range(0..moves.len())];
        m.print();

        game_state.make_move(m);
    }

    game_state.board_state.piece_board.print();
}