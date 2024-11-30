use barschbot::{board::bit_array::{self, BitArray}, game::game_state::GameState};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
fn main() {
    let mut game_state = GameState::from_fen("rnbqkbnr/pppp1ppp/4p3/3N4/8/8/PPPPPPPP/R1BQKBNR b KQkq -");

    let moves = game_state.gen_legal_moves();
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