// use crate::board::piece_type::ColoredPieceType;

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
