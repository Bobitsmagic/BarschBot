use crate::{board::{dynamic_state::DynamicState, piece_board::PieceBoard, piece_type::ColoredPieceType, player_color::PlayerColor, square::Square}, game::game_flags::GameFlags};

pub fn from_fen(fen: &str) -> (PieceBoard, GameFlags) {
    let parts = fen.split(" ").collect::<Vec<_>>();
    let mut board = PieceBoard::empty();
    let mut square = 64 - 8;

    const ALL_CHARS: &str = "rnbqkpRNBQKP12345678/";

    for c in parts[0].chars() {
        if ALL_CHARS.find(c) == None {
            continue;
        }
        if c == '/' {
            square -= 16;
            continue;
        }
        
        let piece = ColoredPieceType::from_char(c);
        
        if piece != ColoredPieceType::None {
            board.add_piece(piece,  Square::from_u8(square));
            
            square += 1;
        }
        else {
            square += c.to_string().parse::<u8>().unwrap();
        }
    }

    let mut game_flags = GameFlags::empty_flags();
    game_flags.active_color = match parts[1] {
        "w" => PlayerColor::White,
        "b" => PlayerColor::Black,
        _ => panic!("Invalid active color")
    };

    for c in parts[2].chars() {
        match c {
            'K' => game_flags.white_king_side_castle = true,
            'Q' => game_flags.white_queen_side_castle = true,
            'k' => game_flags.black_king_side_castle = true,
            'q' => game_flags.black_queen_side_castle = true,
            _ => ()
        }
    }

    if parts[3] != "-" {
        game_flags.en_passant_square = Square::from_str(parts[3]);
    }

    return (board, game_flags);
}

pub fn to_fen(board: &PieceBoard, game_flags: &GameFlags) -> String {
    let mut fen = String::new();
    let mut empty_count = 0;

    for rank in (0..8).rev() {
        for file in 0..8 {
            let square = Square::from_rank_file_index(rank, file);
            let piece = board[square];

            if piece == ColoredPieceType::None {
                empty_count += 1;
            }
            else {
                if empty_count > 0 {
                    fen.push_str(&empty_count.to_string());
                    empty_count = 0;
                }
                fen.push(piece.to_char());
            }
        }

        if empty_count > 0 {
            fen.push_str(&empty_count.to_string());
            empty_count = 0;
        }

        if rank > 0 {
            fen.push('/');
        }
    }

    fen.push(' ');

    fen.push_str(&game_flags.active_color.to_string());
    fen.push(' ');

    let mut castle_flags = String::new();
    if game_flags.white_king_side_castle {
        castle_flags.push('K');
    }
    if game_flags.white_queen_side_castle {
        castle_flags.push('Q');
    }
    if game_flags.black_king_side_castle {
        castle_flags.push('k');
    }
    if game_flags.black_queen_side_castle {
        castle_flags.push('q');
    }

    if castle_flags.len() == 0 {
        fen.push('-');
    }
    else {
        fen.push_str(&castle_flags);
    }

    fen.push(' ');

    if game_flags.en_passant_square != Square::None {
        fen.push_str(&game_flags.en_passant_square.to_string());
    }
    else {
        fen.push('-');
    }

    return fen;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fen() {
        const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -";
        let (board, game_flags) = from_fen(START_FEN);
        assert_eq!(to_fen(&board, &game_flags), START_FEN);
    }
}