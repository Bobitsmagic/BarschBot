use barschbot::board::{chess_move::ChessMove, color::Color, piece_board, piece_type::ColoredPieceType, square::Square};
fn main() {
    let mut board = piece_board::PieceBoard::start_position();

    board.print_perspective(Color::White);

    board.make_move(ChessMove::new(Square::E2, Square::E4, ColoredPieceType::WhitePawn, ColoredPieceType::None));

    board.print_perspective(Color::White);
}