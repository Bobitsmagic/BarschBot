use barschbot::{board::{bit_board::{self}, color::PlayerColor, piece_board, piece_type::ColoredPieceType, square::Square}, game::{game_flags::GameFlags, game_state::GameState}, moves::{self, chess_move::ChessMove}};
fn main() {
    let mut game_state = GameState::start_position();
    game_state.board_state.piece_board.print();

    game_state.make_move(ChessMove::new(Square::E2, Square::E4, ColoredPieceType::WhitePawn, ColoredPieceType::None));

    game_state.board_state.piece_board.print();
}