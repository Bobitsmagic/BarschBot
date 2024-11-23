use crate::{board::{bit_board::BitBoard, dynamic_state::DynamicState, piece_board::PieceBoard, piece_type::ColoredPieceType, player_color::PlayerColor, square::Square}, moves::chess_move::ChessMove};

#[derive(Clone)]
pub struct BoardState {
    pub piece_board: PieceBoard,
    pub bit_board: BitBoard,
}

impl BoardState {
    pub fn start_position() -> BoardState {
        BoardState {
            piece_board: PieceBoard::start_position(),
            bit_board: BitBoard::from_piece_board(&PieceBoard::start_position()),
        }
    }

    pub fn is_in_check(&self, color: PlayerColor) -> bool {
        return self.bit_board.is_in_check(color);        
    }
}

impl DynamicState for BoardState {    
    fn empty() -> Self {
        BoardState {
            piece_board: PieceBoard::empty(),
            bit_board: BitBoard::empty(),
        }
    }
    
    fn add_piece(&mut self, pt: ColoredPieceType, s: Square) {
        self.piece_board.add_piece(pt, s);
        self.bit_board.add_piece(pt, s);
    }
    
    fn remove_piece(&mut self, pt: ColoredPieceType, s: Square) {
        self.piece_board.remove_piece(pt, s);
        self.bit_board.remove_piece(pt, s);
    }

    fn make_move(&mut self, m: ChessMove) {
        self.piece_board.make_move(m);
        self.bit_board.make_move(m);
    }

    fn undo_move(&mut self, m: ChessMove) {
        self.piece_board.undo_move(m);
        self.bit_board.undo_move(m);
    }
}