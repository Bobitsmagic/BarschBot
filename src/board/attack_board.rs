use super::{bit_array::BitArray, bit_board::BitBoard, piece_type::ColoredPieceType, square::Square};

pub struct AttackBoard {
    white_attacks: BitArray,
    black_attacks: BitArray,
    pawn_attacks: BitArray,
    knight_attacks: BitArray,
    diagonal_attacks: BitArray,
    orhtogonal_attacks: BitArray,
    king_attacks: BitArray,
}

impl AttackBoard {
    pub fn empty() -> AttackBoard {
        AttackBoard {
            white_attacks: BitArray::empty(),
            black_attacks: BitArray::empty(),
            pawn_attacks: BitArray::empty(),
            knight_attacks: BitArray::empty(),
            diagonal_attacks: BitArray::empty(),
            orhtogonal_attacks: BitArray::empty(),
            king_attacks: BitArray::empty(),
        }
    }

    pub fn add_piece(&mut self, cpt: ColoredPieceType, sq: Square, bb: &BitBoard) {
        unimplemented!()
    }
}


