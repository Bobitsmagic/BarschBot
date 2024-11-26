use barschbot::{board::{piece_type::ColoredPieceType, square::Square}, game::game_state::GameState, moves::{chess_move::ChessMove, pseudo_move_gen}};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
fn main() {
    let mut game_state = GameState::start_position();

    let moves = game_state.gen_legal_moves();



    game_state.make_move(moves[12]);

    game_state.board_state.piece_board.print();

    let moves = game_state.gen_legal_moves();

    game_state.make_move(moves[13]);

    game_state.board_state.piece_board.print();

    let moves = game_state.gen_legal_moves();
    for m in moves.iter() {
        m.print();
    }
    
    game_state.make_move(moves[0]);

    game_state.board_state.piece_board.print();
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