use arrayvec::ArrayVec;

use crate::board::{bit_array::BitArray, bit_array_lookup::ROWS, piece_board::PieceBoard, piece_type::PieceType, square::Square};

use super::chess_move::ChessMove;

type SquareVec = ArrayVec<Square, 16>;
type BitBoardVec = ArrayVec<u64, 16>;
pub struct MoveIterator {
    squares: SquareVec,
    bit_boards: BitBoardVec,
}

impl MoveIterator {
    pub fn new() -> MoveIterator {
        MoveIterator {
            squares: SquareVec::new(),
            bit_boards: BitBoardVec::new(),
        }
    }

    pub fn add_move(&mut self, square: Square, bit_board: u64) {
        self.squares.push(square);
        self.bit_boards.push(bit_board);
    }

    pub fn iterate_uci_squares(&self) -> impl Iterator<Item=(Square, Square)> + '_ {
        return self.squares.iter().zip(self.bit_boards.iter()).flat_map(|(start_square, bit_board)| {
            bit_board.iterate_squares().map(|target_square| {
                (*start_square, target_square)
            })
        });
    }

    pub fn iterate_moves(&self, piece_board: &PieceBoard) -> impl Iterator<Item=ChessMove> {
        return self.squares.iter().zip(self.bit_boards.iter()).flat_map(|(start_square, bit_board)| {
            let pt = piece_board[*start_square];

            if pt.is_pawn() {
                let promotions = bit_board & (ROWS[0] | ROWS[7]);
                let non_promotions = bit_board & !promotions;

                promotions.iterate_squares().flat_map(|target_square| {
                    [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight].map(move |promotion| {
                        ChessMove::new_pawn(*start_square, target_square, pt,
                            piece_board[target_square], promotion.colored(pt.color()))
                        })
                    })
                .chain(non_promotions.iterate_squares().map(|target_square| {
                    ChessMove::new(*start_square, target_square, pt, piece_board[target_square])
                }))
            }
            else {
                bit_board.iterate_squares().map(|target_square| {
                    ChessMove::new(*start_square, target_square, pt, piece_board[target_square])
                })
            }
        });
    }
}