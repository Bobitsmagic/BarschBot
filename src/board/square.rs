use crate::board::bit_array::BitArray;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1, 
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8, None
}

pub const ALL_SQUARES: [Square; 65] = [
    Square::A1, Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1, Square::H1,
    Square::A2, Square::B2, Square::C2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2,
    Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3,
    Square::A4, Square::B4, Square::C4, Square::D4, Square::E4, Square::F4, Square::G4, Square::H4,
    Square::A5, Square::B5, Square::C5, Square::D5, Square::E5, Square::F5, Square::G5, Square::H5,
    Square::A6, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6,
    Square::A7, Square::B7, Square::C7, Square::D7, Square::E7, Square::F7, Square::G7, Square::H7,
    Square::A8, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8, Square::H8, 
    Square::None
];

const RANK_NAMES: [&str; 8] = ["1", "2", "3", "4", "5", "6", "7", "8"];
const FILE_NAMES: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    R1, R2, R3, R4, R5, R6, R7, R8
}

pub const ALL_RANKS: [Rank; 8] = [Rank::R1, Rank::R2, Rank::R3, Rank::R4, Rank::R5, Rank::R6, Rank::R7, Rank::R8];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum File {
    A, B, C, D, E, F, G, H
}

pub const ALL_FILES: [File; 8] = [File::A, File::B, File::C, File::D, File::E, File::F, File::G, File::H];

impl ToString for Square {
    fn to_string(&self) -> String {
        if self.is_valid() {
            let rank = self.rank_index();
            let file = self.file_index();
            format!("{}{}", FILE_NAMES[file as usize], RANK_NAMES[rank as usize])
        }
        else {
            "None".to_string()
        }

    }
}

impl Square {
    //Constructor
    pub fn from_rank_file(rank: Rank, file: File) -> Square {
        Square::from_rank_file_index(rank as u8, file as u8)
    }
    pub fn from_rank_file_index(rank: u8, file: u8) -> Square {
        debug_assert!(rank < 8, "Invalid rank: {}", rank);
        debug_assert!(file < 8, "Invalid file: {}", file);

        Square::from_u8(rank * 8 + file)
    }

    //[TODO] Benchmark array vs match vs transmute
    pub fn from_u8(index: u8) -> Square {
        Square::from_usize(index as usize)
    }

    pub fn from_usize(index: usize) -> Square {
        if index < ALL_SQUARES.len() {
            ALL_SQUARES[index]
        } else {
            panic!("Invalid square index: {}", index);
        }
    }

    //Methods
    pub fn is_valid(&self) -> bool {
        *self != Square::None
    }

    pub fn rank(&self) -> Rank {
        debug_assert!(*self != Square::None, "Invalid square: None");

        match self.rank_index() {
            0 => Rank::R1,
            1 => Rank::R2,
            2 => Rank::R3,
            3 => Rank::R4,
            4 => Rank::R5,
            5 => Rank::R6,
            6 => Rank::R7,
            7 => Rank::R8,
            _ => unreachable!()
        }
    }

    pub fn file(&self) -> File {
        debug_assert!(*self != Square::None, "Invalid square: None");

        match self.file_index() {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => unreachable!()
        }
    }

    pub fn rank_index(&self) -> u8 {
        debug_assert!(*self != Square::None, "Invalid square: None");

        (*self as u8) / 8
    }

    pub fn file_index(&self) -> u8 {
        debug_assert!(*self != Square::None, "Invalid square: None");

        (*self as u8) % 8
    }

    pub fn get_bitarray(&self) -> BitArray {
        debug_assert!(*self != Square::None, "Invalid square: None");

        BitArray { bits: 1 << (*self as u8) }
    }

    pub fn is_light(&self) -> bool {
        let rank = self.rank_index();
        let file = self.file_index();

        (rank + file) % 2 == 0
    }
    
    pub fn rectangle_to(&self, other: Square) -> impl Iterator<Item = Square> {
        let r1 = self.rank_index();
        let r2 = other.rank_index();
        let min_rank = r1.min(r2);
        let max_rank = r1.max(r2);


        let f1 = self.file_index();
        let f2 = other.file_index();
        let min_file = f1.min(f2);
        let max_file = f1.max(f2);

        let rank_range = min_rank..=max_rank;
        let file_range = min_file..=max_file;

        rank_range.flat_map(move |rank| file_range.clone().map(move |file| Square::from_rank_file_index(rank, file)))
    }

    pub fn to_smybol(&self) -> &'static str {
        if self.is_light() {
            "□"
        } else {
            "■"
        }
    }
    
    pub fn up(&self) -> Square {
        let rank = self.rank_index();
        let file = self.file_index();

        assert!(rank < 7, "Invalid rank: {}", rank);

        return Square::from_rank_file_index(rank + 1, file);
    }

    pub fn down(&self) -> Square {
        let rank = self.rank_index();
        let file = self.file_index();

        assert!(rank > 0, "Invalid rank: {}", rank);

        return Square::from_rank_file_index(rank - 1, file);
    }

    pub fn left(&self) -> Square {
        let rank = self.rank_index();
        let file = self.file_index();

        assert!(file > 0, "Invalid file: {}", file);

        return Square::from_rank_file_index(rank, file - 1);
    }

    pub fn right(&self) -> Square {
        let rank = self.rank_index();
        let file = self.file_index();

        assert!(file < 7, "Invalid file: {}", file);

        return Square::from_rank_file_index(rank, file + 1);
    }
}

#[cfg(test)]
mod square_tests {
    use super::*;

    #[test]
    fn test_from_index() {
        for i in 0..65 {
            let square = Square::from_usize(i);
            assert_eq!(square as u8, i as u8);
        }
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Square::A1.to_string(), "a1");
        assert_eq!(Square::C4.to_string(), "c4");
        assert_eq!(Square::H8.to_string(), "h8");
    }

    #[test]
    fn test_rank_file() {
        for x in 0..8 {
            for y in 0..8 {
                let square = Square::from_rank_file_index(x, y);
                assert_eq!(square.rank_index(), x);
                assert_eq!(square.file_index(), y);
            }
        }
    }
}