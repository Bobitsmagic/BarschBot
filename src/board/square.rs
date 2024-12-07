use std::ops::Range;

pub const A1: i8 = 0;
pub const B1: i8 = 1;
pub const C1: i8 = 2;
pub const D1: i8 = 3;
pub const E1: i8 = 4;
pub const F1: i8 = 5;
pub const G1: i8 = 6;
pub const H1: i8 = 7;

pub const A2: i8 = 8;
pub const B2: i8 = 9;
pub const C2: i8 = 10;
pub const D2: i8 = 11;
pub const E2: i8 = 12;
pub const F2: i8 = 13;
pub const G2: i8 = 14;
pub const H2: i8 = 15;

pub const A3: i8 = 16;
pub const B3: i8 = 17;
pub const C3: i8 = 18;
pub const D3: i8 = 19;
pub const E3: i8 = 20;
pub const F3: i8 = 21;
pub const G3: i8 = 22;
pub const H3: i8 = 23;

pub const A4: i8 = 24;
pub const B4: i8 = 25;
pub const C4: i8 = 26;
pub const D4: i8 = 27;
pub const E4: i8 = 28;
pub const F4: i8 = 29;
pub const G4: i8 = 30;
pub const H4: i8 = 31;

pub const A5: i8 = 32;
pub const B5: i8 = 33;
pub const C5: i8 = 34;
pub const D5: i8 = 35;
pub const E5: i8 = 36;
pub const F5: i8 = 37;
pub const G5: i8 = 38;
pub const H5: i8 = 39;

pub const A6: i8 = 40;
pub const B6: i8 = 41;
pub const C6: i8 = 42;
pub const D6: i8 = 43;
pub const E6: i8 = 44;
pub const F6: i8 = 45;
pub const G6: i8 = 46;
pub const H6: i8 = 47;

pub const A7: i8 = 48;
pub const B7: i8 = 49;
pub const C7: i8 = 50;
pub const D7: i8 = 51;
pub const E7: i8 = 52;
pub const F7: i8 = 53;
pub const G7: i8 = 54;
pub const H7: i8 = 55;

pub const A8: i8 = 56;
pub const B8: i8 = 57;
pub const C8: i8 = 58;
pub const D8: i8 = 59;
pub const E8: i8 = 60;
pub const F8: i8 = 61;
pub const G8: i8 = 62;
pub const H8: i8 = 63;

pub const NONE: i8 = 64;

pub const VALID_SQUARES: Range<i8> = 0_i8..64;


pub const EN_PASSANT_SQUARES: [i8; 16] = [
    A3, B3, C3, D3, E3, F3, G3, H3,
    A6, B6, C6, D6, E6, F6, G6, H6,
];

const RANK_NAMES: [&str; 8] = ["1", "2", "3", "4", "5", "6", "7", "8"];
const FILE_NAMES: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

pub trait Square {
    fn is_valid_square(self) -> bool;
    fn rank(self) -> i8;
    fn file(self) -> i8;
    fn is_light(self) -> bool;
    fn is_dark(self) -> bool;
    fn square_string(self) -> String;
    fn bit_array(self) -> u64;
    fn up(self) -> Self;
    fn down(self) -> Self;
    fn left(self) -> Self;
    fn right(self) -> Self;
    fn translate(&self, dx: i8, dy: i8) -> Self;
    fn flip_x(self) -> Self;
    fn flip_y(self) -> Self;
    fn rotate_180(self) -> Self;
}

pub fn from_file_rank(file: i8, rank: i8) -> i8 {
    debug_assert!(rank >= 0 && rank < 8, "Invalid rank: {}", rank);
    debug_assert!(file >= 0 && file < 8, "Invalid file: {}", file);

    file | (rank << 3)
}

pub fn from_str(s: &str) -> i8 {
    let file = s.chars().nth(0).unwrap();
    let rank = s.chars().nth(1).unwrap();

    let file = match file {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => panic!("Invalid file: {}", file)
    };

    let rank = match rank {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        _ => panic!("Invalid rank: {}", rank)
    };

    from_file_rank(file, rank)
}

impl Square for i8 {
    fn is_valid_square(self) -> bool {
        self >= A1 && self <= H8
    }

    fn is_light(self) -> bool {
        let rank = self.rank();
        let file = self.file();

        (rank + file) % 2 == 1
    }

    fn is_dark(self) -> bool {
        let rank = self.rank();
        let file = self.file();

        (rank + file) % 2 == 0
    }

    fn rank(self) -> i8 {
        debug_assert!(self.is_valid_square(), "Invalid square: {}", self);

        self >> 3
    }

    fn file(self) -> i8 {
        debug_assert!(self.is_valid_square(), "Invalid square: {}", self);

        self & 7
    }

    fn square_string(self) -> String {
        if self.is_valid_square() {
            let rank = self.rank();
            let file = self.file();
            format!("{}{}", FILE_NAMES[file as usize], RANK_NAMES[rank as usize])
        }
        else {
            "None".to_string()
        }
    }

    fn bit_array(self) -> u64 {
        debug_assert!(self.is_valid_square(), "Invalid square: {}", self);

        1_u64 << self
    }

    fn up(self) -> i8 {
        debug_assert!((self + 8).is_valid_square(), "Square cant be shifted up: {}", self.square_string());

        self + 8
    }
    fn down(self) -> i8 {
        debug_assert!((self - 8).is_valid_square(), "Square cant be shifted down: {}", self.square_string());

        self - 8
    }
    fn left(self) -> i8 {
        debug_assert!((self - 1).is_valid_square(), "Square cant be shifted left: {}", self.square_string());

        self - 1
    }
    fn right(self) -> i8 {
        debug_assert!((self + 1).is_valid_square(), "Square cant be shifted right: {}", self.square_string());

        self + 1
    }

    fn flip_x(self) -> i8 {
        self ^ 7
    }
    fn flip_y(self) -> i8 {
        self ^ 56 // 7 * 8
    }

    fn rotate_180(self) -> i8 {
        self ^ 63 // 7 * 8 + 7
    }

    fn translate(&self, dx: i8, dy: i8) -> i8 {
        let delta = dx + (dy * 8);

        debug_assert!((self + delta).is_valid_square(), "Invalid square after translation: {} dx: {} dy: {}", self.square_string(), dx, dy);

        self + delta
    }
}

#[cfg(test)]
mod square_tests {
    use crate::board::square;

    use super::*;

    #[test]
    fn test_from_index() {
        for i in 0..65 {
            let square = i as i8;
            assert_eq!(square as u8, i as u8);
        }
    }

    #[test]
    fn test_to_string() {
        assert_eq!(A1.square_string(), "a1");
        assert_eq!(C4.square_string(), "c4");
        assert_eq!(H8.square_string(), "h8");
    }

    #[test]
    fn test_rank_file() {
        for x in 0..8 {
            for y in 0..8 {
                let square = square::from_file_rank(x, y);
                assert_eq!(square.file(), x);
                assert_eq!(square.rank(), y);
            }
        }
    }

    #[test]
    fn test_flips() {
        for x in 0..8 {
            for y in 0..8 {
                let square = square::from_file_rank(x, y);
                assert_eq!(square.flip_x().flip_x(), square);
                assert_eq!(square.flip_y().flip_y(), square);
                assert_eq!(square.rotate_180().rotate_180(), square);

                assert_eq!(square.flip_x().file(), 7 - x);
                assert_eq!(square.flip_y().rank(), 7 - y);
                assert_eq!(square.rotate_180().file(), 7 - x);
                assert_eq!(square.rotate_180().rank(), 7 - y);
            }
        }
    }
}