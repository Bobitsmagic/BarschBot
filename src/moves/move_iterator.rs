use arrayvec::ArrayVec;

use crate::board::{bit_array::BitArray, piece_board::PieceBoard, piece_type::PieceType, player_color::PlayerColor, rank, square::Square};

use super::chess_move::ChessMove;

type SquareVec = ArrayVec<i8, 16>;
type BitBoardVec = ArrayVec<u64, 16>;
pub struct MoveIterator {
    squares: SquareVec,
    bit_boards: BitBoardVec,
    left_pawn_captures: u64,
    right_pawn_captures: u64,
    pawn_pushes: u64,
    double_pushes: u64,
}

impl MoveIterator {
    pub fn new() -> MoveIterator {
        MoveIterator {
            squares: SquareVec::new(),
            bit_boards: BitBoardVec::new(),
            pawn_pushes: 0,
            double_pushes: 0,
            left_pawn_captures: 0,
            right_pawn_captures: 0,
        }
    }

    pub fn print(&self) {
        for (square, bit_board) in self.squares.iter().zip(self.bit_boards.iter()) {
            if *bit_board == 0 {
                continue;
            }
            println!("Square: {}", square.to_string());
            bit_board.print();
        }
        // println!("Pawn Pushes: {}", BitArray::new(self.pawn_pushes));
        // println!("Double Pushes: {}", BitArray::new(self.double_pushes));
        // println!("Left Pawn Captures: {}", BitArray::new(self.left_pawn_captures));
        // println!("Right Pawn Captures: {}", BitArray::new(self.right_pawn_captures));
    }

    pub fn add_move(&mut self, square: i8, bit_board: u64) {
        self.squares.push(square);
        self.bit_boards.push(bit_board);
    }
    pub fn add_pawn_push(&mut self, bit_board: u64) {
        self.pawn_pushes |= bit_board;
    }
    pub fn add_double_pawn_push(&mut self, bit_board: u64) {
        self.double_pushes |= bit_board;
    }
    pub fn add_pawn_left_capture(&mut self, bit_board: u64) {
        self.left_pawn_captures |= bit_board;
    }
    pub fn add_pawn_right_capture(&mut self, bit_board: u64) {
        self.right_pawn_captures |= bit_board;
    }

    pub fn count_moves(&self) -> u32 {
        let mut sum = 0;
        
        for bit_board in self.bit_boards.iter() {
            sum += bit_board.count_ones();
        }
        sum += self.double_pushes.count_ones();
        
        sum += count_pawn_moves(self.pawn_pushes);
        sum += count_pawn_moves(self.left_pawn_captures);
        sum += count_pawn_moves(self.right_pawn_captures);
        
        return sum;
        
        fn count_pawn_moves(moves: u64) -> u32 {
            const LAST_RANK: u64 = 0xFF000000000000FF;
            return (moves & !LAST_RANK).count_ones() + (moves & LAST_RANK).count_ones() * 4;
        }
    }

    pub fn iterate_uci_squares(&self, color: PlayerColor) -> impl Iterator<Item=(i8, i8)> + '_ {
        let dy = match color {
            PlayerColor::White => -1,
            PlayerColor::Black => 1,
        };
        return self.squares.iter().zip(self.bit_boards.iter()).flat_map(|(start_square, bit_board)| {
            bit_board.iterate_squares().map(|target_square| {
                (*start_square, target_square)
            })
        }).chain(
            self.pawn_pushes.iterate_squares().map(move |target_square| {
                (target_square.translate(0, dy), target_square)
            })
        ).chain(
            self.double_pushes.iterate_squares().map(move |target_square| {
                (target_square.translate(0, 2 * dy), target_square)
            })
        ).chain(
            self.left_pawn_captures.iterate_squares().map(move |target_square| {
                (target_square.translate(1, dy), target_square)
            })
        ).chain(
            self.right_pawn_captures.iterate_squares().map(move |target_square| {
                (target_square.translate(-1, dy), target_square)
            })
        );
    }

    pub fn iterate_piece_squares(&self) -> impl Iterator<Item=(i8, i8)> + '_ {
        return self.squares.iter().zip(self.bit_boards.iter()).flat_map(|(start_square, bit_board)| {
            bit_board.iterate_squares().map(|target_square| {
                (*start_square, target_square)
            })
        });
    }

    pub fn iterate_pawn_squares(&self, color: PlayerColor) -> impl Iterator<Item=(i8, i8)> + '_ {
        let dy = match color {
            PlayerColor::White => -1,
            PlayerColor::Black => 1,
        };
        return self.pawn_pushes.iterate_squares().map(move |target_square| {
            (target_square.translate(0, dy), target_square)
        }).chain(
            self.double_pushes.iterate_squares().map(move |target_square| {
                (target_square.translate(0, 2 * dy), target_square)
            })
        ).chain(
            self.left_pawn_captures.iterate_squares().map(move |target_square| {
                (target_square.translate(1, dy), target_square)
            })
        ).chain(
            self.right_pawn_captures.iterate_squares().map(move |target_square| {
                (target_square.translate(-1, dy), target_square)
            })
        );
    }

    pub fn iterate_piece_moves<'a>(&'a self, piece_board: &'a PieceBoard) -> impl Iterator<Item=ChessMove> + '_ {
        return self.squares.iter().zip(self.bit_boards.iter()).flat_map(move |(start_square, bit_board)| {
            bit_board.iterate_squares().map(move |target_square| {
                let pt = piece_board[*start_square];
                let cpt = piece_board[target_square];
                return ChessMove::new(*start_square, target_square, pt, cpt);
            })
        });
    }

    pub fn iterate_pawn_moves<'a>(&'a self, piece_board: &'a PieceBoard, color: PlayerColor) -> impl Iterator<Item=ChessMove> + '_ {
        let dy = match color {
            PlayerColor::White => -1,
            PlayerColor::Black => 1,
        };

        return self.pawn_pushes.iterate_squares().map(move |target_square| {
            (target_square.translate(0, dy), target_square)
        }).chain(
        self.left_pawn_captures.iterate_squares().map(move |target_square| {
            (target_square.translate(1, dy), target_square)
        })).chain(
        self.right_pawn_captures.iterate_squares().map(move |target_square| {
            (target_square.translate(-1, dy), target_square)
        })).flat_map(move |(start_square, target_square)| {
            let pt = piece_board[start_square];
            let cpt = piece_board[target_square];

            if target_square.rank() == rank::R1 || target_square.rank() == rank::R8 {
                return vec![
                    ChessMove::new_pawn(start_square, target_square, pt, cpt, PieceType::Queen.colored(color)),
                    ChessMove::new_pawn(start_square, target_square, pt, cpt, PieceType::Rook.colored(color)),
                    ChessMove::new_pawn(start_square, target_square, pt, cpt, PieceType::Bishop.colored(color)),
                    ChessMove::new_pawn(start_square, target_square, pt, cpt, PieceType::Knight.colored(color)),
                ].into_iter();
            }
            else {
                return vec![ChessMove::new(start_square, target_square, pt, cpt)].into_iter();
            }
        }).chain(
            self.double_pushes.iterate_squares().map(move |target_square| {
                let start_square = target_square.translate(0, dy * 2);
                let pt = piece_board[start_square];
                let cpt = piece_board[target_square];

                return ChessMove::new(start_square, target_square, pt, cpt);
        }));
    }

    pub fn iterate_all_moves<'a>(&'a self, piece_board: &'a PieceBoard, color: PlayerColor) -> impl Iterator<Item=ChessMove> + '_ {
        return self.iterate_piece_moves(piece_board).chain(self.iterate_pawn_moves(piece_board, color));
    }

    pub fn iterate_square_moves<'a>(&'a self, piece_board: &'a PieceBoard, color: PlayerColor) -> impl Iterator<Item=ChessMove> + 'a {
        return self.iterate_uci_squares(color).flat_map(move |(start_square, target_square)| {
            let pt = piece_board[start_square];
            let cpt = piece_board[target_square];
            if pt.is_pawn() {
                if target_square.rank() == rank::R1 || target_square.rank() == rank::R8 {
                    return vec![
                        ChessMove::new_pawn(start_square, target_square, pt, cpt, PieceType::Queen.colored(color)),
                        ChessMove::new_pawn(start_square, target_square, pt, cpt, PieceType::Rook.colored(color)),
                        ChessMove::new_pawn(start_square, target_square, pt, cpt, PieceType::Bishop.colored(color)),
                        ChessMove::new_pawn(start_square, target_square, pt, cpt, PieceType::Knight.colored(color)),
                    ].into_iter();
                }
                else {
                    return vec![ChessMove::new(start_square, target_square, pt, cpt)].into_iter();
                }
            }
            else {
                return vec![ChessMove::new(start_square, target_square, pt, cpt)].into_iter();
            }
        });
    }
}