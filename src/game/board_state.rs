use crate::{board::{bit_board::BitBoard, dynamic_state::DynamicState, piece_board::PieceBoard, piece_type::ColoredPieceType, player_color::PlayerColor, square::{Square, VALID_SQUARES}}, moves::chess_move::ChessMove};

#[derive(Clone, PartialEq, Eq)]
pub struct BoardState {
    pub piece_board: PieceBoard,
    pub bit_board: BitBoard,
}

impl BoardState {
    pub fn start_position() -> BoardState {
        BoardState::from_piece_board(&PieceBoard::start_position())
    }

    pub fn from_piece_board(piece_board: &PieceBoard) -> BoardState {
        BoardState {
            piece_board: piece_board.clone(),
            bit_board: BitBoard::from_piece_board(piece_board),
        }
    }

    pub fn is_in_check(&self, color: PlayerColor) -> bool {
        return self.bit_board.is_in_check(color);        
    }

    pub fn square_attacked(&self, s: Square, opponent_color: PlayerColor) -> bool {
        return self.bit_board.square_is_attacked_by(s, opponent_color);
    }

    pub fn check_integrity(&self) -> bool {
        for s in VALID_SQUARES {
            if self.piece_board[s] != self.bit_board.get_colored_piecetype(s) {
                println!("{:?}", s);
                println!("  {:?}", self.piece_board[s]);
                println!("  {:?}", self.bit_board.get_colored_piecetype(s));

                return false;
            }
        }

        return true;
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

#[cfg(test)]
mod tests {
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    use crate::{board::square::VALID_SQUARES, game::game_flags::GameFlags, moves::{move_gen}};

    use super::*;

    #[test]
    fn test_start_position() {
        let board_state = BoardState::start_position();
        
        for s in VALID_SQUARES {
            // println!("{:?} {:?}", s, board_state.piece_board[s]);
            // println!("{:?} {:?}", s, board_state.bit_board.get_colored_piecetype(s));

            assert_eq!(board_state.piece_board[s], board_state.bit_board.get_colored_piecetype(s));
        }
    }

    #[test]
    fn test_random_moves() {
        let mut board_state = BoardState::start_position();
        let mut game_flags = GameFlags::start_flags();
        
        let mut rng = ChaCha8Rng::seed_from_u64(0);
        for _ in 0..30 {
            let moves = move_gen::gen_legal_moves(&board_state, &game_flags);
            if moves.is_empty() {
                break;
            }

            let m = moves[rng.gen_range(0..moves.len())];
            
            m.print();
            
            board_state.make_move(m);
            game_flags.make_move(m, &board_state.bit_board);

            board_state.piece_board.print();

            for s in VALID_SQUARES {    
                if board_state.piece_board[s] != board_state.bit_board.get_colored_piecetype(s) {
                    println!("{:?}", s);
                    println!("  {:?}", board_state.piece_board[s]);
                    println!("  {:?}", board_state.bit_board.get_colored_piecetype(s));

                    panic!();
                }
            }
        }
    }

    #[test]
    fn test_undo_moves() {
        let mut board_state = BoardState::start_position();
        let mut game_flags = GameFlags::start_flags();
        
        let mut rng = ChaCha8Rng::seed_from_u64(1);
        for i in 0..60 {
            let moves = move_gen::gen_legal_moves(&board_state, &game_flags);
            if moves.is_empty() {
                break;
            }

            let m = moves[rng.gen_range(0..moves.len())];
            
            m.print();
            
            board_state.make_move(m);
            
            if i % 2 == 0 {
                board_state.undo_move(m);
            }
            else {
                game_flags.make_move(m, &board_state.bit_board);
            }
            
            board_state.piece_board.print();
            for s in VALID_SQUARES {    
                if board_state.piece_board[s] != board_state.bit_board.get_colored_piecetype(s) {
                    println!("{:?}", s);
                    println!("  {:?}", board_state.piece_board[s]);
                    println!("  {:?}", board_state.bit_board.get_colored_piecetype(s));

                    panic!();
                }
            }
        }
    }
}