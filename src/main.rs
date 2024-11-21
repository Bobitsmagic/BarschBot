use barschbot::{board::{bit_array_lookup, bit_board::{self, BitBoard}, color::PlayerColor, piece_board, piece_type::ColoredPieceType, square::Square}, game::game_state::{self, GameState}, moves::{self, chess_move::ChessMove}};
fn main() {
    let mut piece_board = piece_board::PieceBoard::start_position();
    let mut bit_board = bit_board::BitBoard::from_piece_board(&piece_board);
    let mut game_state = GameState::new();

    piece_board.print();

    let moves = moves::move_gen::generate_pseudo_legal_moves_bitboard(&bit_board, &piece_board, &game_state, PlayerColor::White);

    println!("Count: {}", moves.len());
    for m in moves {
        m.print();  
    }
}