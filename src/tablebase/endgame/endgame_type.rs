use crate::board::{
    dynamic_state::DynamicState,
    piece_type::{ColoredPieceType, PieceType},
};

pub const NO_KING_PIECES: [ColoredPieceType; 10] = [
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

const INDEX_MAP: [usize; 13] = [
    0,
    1,
    2,
    3,
    4,
    usize::MAX,
    5,
    6,
    7,
    8,
    9,
    usize::MAX,
    usize::MAX,
];

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EndGameType {
    pub pieces: [i8; 10],
}

impl Ord for EndGameType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.piece_count()
            .cmp(&other.piece_count())
            .then(self.pawn_count().cmp(&other.pawn_count()))
            .then(other.pieces.cmp(&self.pieces))
    }
}

impl PartialOrd for EndGameType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl EndGameType {
    pub fn gen_successors(&self) -> Vec<EndGameType> {
        let mut ret = Vec::new();

        //captures
        for i in 0..10 {
            if self.pieces[i] > 0 {
                let mut next = self.clone();
                next.pieces[i] -= 1;
                ret.push(next);
            }
        }

        //promotions
        for pawn_type in [0, 5] {
            if self.pieces[pawn_type] <= 0 {
                continue;
            }

            for promo_offset in 1..=4 {
                let mut next = self.clone();
                next.pieces[pawn_type] -= 1;
                next.pieces[pawn_type + promo_offset] += 1;
                ret.push(next);
            }
        }

        //promotion capture
        for pawn_type in [0, 5] {
            if self.pieces[pawn_type] <= 0 {
                continue;
            }

            for promo_offset in 1..=4 {
                for capture_type in 0..10 {
                    if pawn_type / 5 == capture_type / 5 {
                        continue;
                    }

                    if self.pieces[capture_type] <= 0 {
                        continue;
                    }

                    let mut next = self.clone();
                    next.pieces[pawn_type] -= 1;
                    next.pieces[capture_type] -= 1;
                    next.pieces[pawn_type + promo_offset] += 1;
                    ret.push(next);
                }
            }
        }

        return ret;
    }

    pub fn representative(&self) -> EndGameType {
        self.clone().min(self.flipped())
    }

    pub fn flipped(&self) -> EndGameType {
        let mut list = [0; 10];
        for i in 0..5 {
            list[i] = self.pieces[i + 5];
            list[i + 5] = self.pieces[i];
        }

        return EndGameType { pieces: list };
    }

    pub fn get_piece_list(&self) -> Vec<ColoredPieceType> {
        let mut list = Vec::new();

        for i in 0..10 {
            for _ in 0..self.pieces[i] {
                list.push(NO_KING_PIECES[i]);
            }
        }

        return list;
    }

    pub fn piece_count(&self) -> i8 {
        self.pieces.iter().sum()
    }
    pub fn pawn_count(&self) -> i8 {
        self.pieces[INDEX_MAP[ColoredPieceType::WhitePawn as usize] as usize]
            + self.pieces[INDEX_MAP[ColoredPieceType::BlackPawn as usize] as usize]
    }
    pub fn has_pawns(&self) -> bool {
        self.pawn_count() > 0
    }

    pub fn is_symmetric(&self) -> bool {
        for i in 0..5 {
            if self.pieces[i] != self.pieces[i + 5] {
                return false;
            }
        }

        return true;
    }

    pub fn get_dif_piece(&self, other: &EndGameType) -> Option<ColoredPieceType> {
        let mut ret_type = None;

        for i in 0..10 {
            match self.pieces[i].abs_diff(other.pieces[i]) {
                0 => continue,
                1 => {
                    if ret_type == None {
                        ret_type = Some(NO_KING_PIECES[i]);
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        }

        return ret_type;
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();

        for i in 0..10 {
            for _ in 0..self.pieces[i] {
                s += &NO_KING_PIECES[i].to_char().to_string();
            }

            if i == 4 {
                s += "_"
            }
        }

        if s == "" {
            s = "K_k".to_owned();
        }

        if s.ends_with("_") {
            s.pop().unwrap();
        }

        return s;
    }
}

impl DynamicState for EndGameType {
    fn empty() -> Self {
        EndGameType { pieces: [0; 10] }
    }

    fn add_piece(&mut self, pt: ColoredPieceType, _: i8) {
        self.pieces[INDEX_MAP[pt as usize]] += 1;
    }

    fn remove_piece(&mut self, pt: ColoredPieceType, _: i8) {
        self.pieces[INDEX_MAP[pt as usize]] -= 1;
    }
}

pub fn generate_piece_lists(max_piece_count: usize) -> Vec<EndGameType> {
    let mut ret = Vec::new();

    backtrack_piece_types(
        &mut EndGameType::empty(),
        0,
        max_piece_count as i8,
        &mut ret,
    );

    return ret;

    fn backtrack_piece_types(
        list: &mut EndGameType,
        min_piece_type: usize,
        max_piece_count: i8,
        ret: &mut Vec<EndGameType>,
    ) {
        ret.push(list.clone());

        if list.piece_count() == max_piece_count {
            return;
        }

        for i in min_piece_type..NO_KING_PIECES.len() {
            list.pieces[i] += 1;

            backtrack_piece_types(list, i, max_piece_count, ret);

            list.pieces[i] -= 1;
        }
    }
}

pub fn generate_piece_lists_sym(max_piece_count: usize) -> Vec<EndGameType> {
    let mut ret = generate_piece_lists(max_piece_count);

    for i in (1..ret.len()).rev() {
        let other = ret[i].flipped();

        if ret[..i].contains(&other) {
            ret.remove(i);
        }
    }

    ret.sort();

    for i in 0..ret.len() {
        ret[i] = ret[i].representative();
    }

    return ret;
}
