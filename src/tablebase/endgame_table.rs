use crate::board::piece_type::ColoredPieceType;

// pub fn piece_list_index(mut list: Vec<ColoredPieceType>) -> u32 {
//     let mut index = 0;
//     list.sort();

//     for i in 0..list.len() {
//         index += list[i].index() << (i * 3);
//     }

//     fn cpt_index(cpt: ColoredPieceType) -> u32 {
//         match cpt {
//             ColoredPieceType::WhitePawn => 0,
//             ColoredPieceType::WhiteKnight => 1,
//             ColoredPieceType::WhiteBishop => 2,
//             ColoredPieceType::WhiteRook => 3,
//             ColoredPieceType::WhiteQueen => 4,

//             ColoredPieceType::BlackPawn => 6,
//             ColoredPieceType::BlackKnight => 7,
//             ColoredPieceType::BlackBishop => 8,
//             ColoredPieceType::BlackRook => 9,
//             ColoredPieceType::BlackQueen => 10,
//         }
//     }
// }

const NO_KING_PIECES: [ColoredPieceType; 10] = [
    ColoredPieceType::WhitePawn,
    ColoredPieceType::WhiteKnight,
    ColoredPieceType::WhiteBishop,
    ColoredPieceType::WhiteRook,
    ColoredPieceType::WhiteQueen,
    ColoredPieceType::BlackPawn,
    ColoredPieceType::BlackKnight,
    ColoredPieceType::BlackBishop,
    ColoredPieceType::BlackRook,
    ColoredPieceType::BlackQueen,
];

pub fn generate_piece_lists(piece_count: usize) -> Vec<Vec<ColoredPieceType>> {
    let mut list = Vec::new();
    let mut ret = Vec::new();

    backtrack_piece_types(&mut list, 0, piece_count, &mut ret);

    return ret;

    fn backtrack_piece_types(
        list: &mut Vec<ColoredPieceType>,
        min_piece_type: usize,
        max_piece_count: usize,
        ret: &mut Vec<Vec<ColoredPieceType>>,
    ) {
        ret.push(list.clone());

        if list.len() == max_piece_count {
            return;
        }

        for i in min_piece_type..NO_KING_PIECES.len() {
            list.push(NO_KING_PIECES[i]);

            backtrack_piece_types(list, i, max_piece_count, ret);

            list.pop();
        }
    }
}
