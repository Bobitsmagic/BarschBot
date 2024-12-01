use barschbot::{board::{piece_type::ColoredPieceType, square}, game::game_state::GameState, moves::chess_move::ChessMove};

fn main() {
    let mut game_state = GameState::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -");

    game_state.make_move(ChessMove::new(square::A2, square::A4, ColoredPieceType::WhitePawn, ColoredPieceType::None));
    
}