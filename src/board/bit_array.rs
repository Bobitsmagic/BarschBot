use std::u64;

use super::{bit_array_lookup::*, square::Square};



#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BitArray {
    pub bits: u64,
}

impl BitArray {
    //Constructor
    pub fn empty() -> BitArray {
        BitArray { bits: 0 }
    }

    pub fn full() -> BitArray {
        BitArray { bits: u64::MAX }
    }

    pub fn new(bits: u64) -> BitArray {
        BitArray { bits }
    }
    
    //Accessors
    pub fn get_bit(&self, square: Square) -> bool {
        self.get_bit_index(square as u8)
    }
    pub fn get_bit_index(&self, index: u8) -> bool {
        (self.bits & (1 << index)) != 0
    }
    pub fn count_bits(&self) -> u8 {
        self.bits.count_ones() as u8
    }
    pub fn to_square(&self) -> Square {
        debug_assert!(self.bits.count_ones() == 1, "Invalid bitboard: {}", self.bits);

        let index = self.bits.trailing_zeros();
        return Square::from_u8(index as u8);
    }
    pub fn print(&self) {
        let mut s = String::new();

        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = Square::from_rank_file_index(rank, file);
                s += &format!("{} ", if self.get_bit(square) { "■" } else { "□" });
            }
            s += "\n";
        }

        println!("{}", s);
    }

    //Mut methods
    pub fn set_bit(&mut self, square: Square) {
        self.set_bit_index(square as u8);
    }
    pub fn set_bit_index(&mut self, index: u8) {
        self.bits |= 1 << index;
    }

    pub fn clear_bit(&mut self, square: Square) {
        self.clear_bit_index(square as u8);
    }
    pub fn clear_bit_index(&mut self, index: u8) {
        self.bits &= !(1 << index);
    }

    pub fn flip_bit(&mut self, square: Square) {
        self.flip_bit_index(square as u8);
    }

    pub fn flip_bit_index(&mut self, index: u8) {
        self.bits ^= 1 << index;
    }

    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    pub fn iterate_squares(&self) -> impl Iterator<Item=Square> {
        self.iterate_set_bits().map(|v| Square::from_usize(v as usize))
    }

    pub fn iterate_set_bits(&self) -> impl Iterator<Item=u32> {
        let mut value = self.bits;
        return std::iter::from_fn(move || {
            if value != 0 {
                let index = value.trailing_zeros();
                value ^= 1_u64 << index;
                
                Some(index)
            }
            else {
                None
            }
        });
    }

    //Translation
    pub fn up(&self) -> BitArray {
        BitArray { bits: self.bits << 8 }
    }

    pub fn down(&self) -> BitArray {
        BitArray { bits: self.bits >> 8 }
    }

    pub fn right(&self) -> BitArray {
        BitArray { bits: (self.bits & !COLLUMNS[7].bits) << 1 }
    }

    pub fn left(&self) -> BitArray {
        BitArray { bits: (self.bits & !COLLUMNS[0].bits) >> 1 }
    }

    pub fn up_right(&self) -> BitArray {
        BitArray { bits: (self.bits & !COLLUMNS[7].bits) << 9 }
    }

    pub fn up_left(&self) -> BitArray {
        BitArray { bits: (self.bits & !COLLUMNS[0].bits) << 7 }
    }

    pub fn down_right(&self) -> BitArray {
        BitArray { bits: (self.bits & !COLLUMNS[7].bits) >> 7 }
    }

    pub fn down_left(&self) -> BitArray {
        BitArray { bits: (self.bits & !COLLUMNS[0].bits) >> 9 }
    }

    pub fn translate(&self, dx: i8, dy: i8) -> BitArray {
        debug_assert!(dx.abs() <= 7 && dy.abs() <= 7, "Invalid translation: ({}, {})", dx, dy);
        
        let mask = if dx >= 0 { ACCUM_COLLUMNS[(7 - dx) as usize] } else { !ACCUM_COLLUMNS[(-dx) as usize - 1] };

        let shift_sum = dx + dy * 8;
        if shift_sum > 0 {
            BitArray { bits: (*self & mask).bits << shift_sum }
        }
        else {
            BitArray { bits: (*self & mask).bits >> -shift_sum }
        }
    }

    pub fn pawn_moves<const WHITE: bool>(&self) -> BitArray {
        if WHITE {
            self.up_left() | self.up_right()
        }
        else {
            self.down_left() | self.down_right()
        }
    }

    //[TODO] benchmark this vs iterate with lookup table
    pub fn knight_moves(&self) -> BitArray {
        self.translate(2, 1) | self.translate(1, 2) | 
        self.translate(-1, 2) | self.translate(-2, 1) |
        self.translate(-2, -1) | self.translate(-1, -2) |
        self.translate(1, -2) | self.translate(2, -1)
    }

    pub fn king_moves(&self) -> BitArray {
        self.up() | self.down() | self.left() | self.right() |
        self.up_left() | self.up_right() | self.down_left() | self.down_right()
    }

    pub fn diagonal_moves(&self) -> BitArray {
        let mut moves = BitArray::empty();

        let mut bb = *self;
        while !bb.is_empty() {
            moves |= bb;
            bb = bb.up_left();
        }

        bb = *self;
        while !bb.is_empty() {
            moves |= bb;
            bb = bb.up_right();
        }

        bb = *self;
        while !bb.is_empty() {
            moves |= bb;
            bb = bb.down_left();
        }

        bb = *self;
        while !bb.is_empty() {
            moves |= bb;
            bb = bb.down_right();
        }

        moves
    }
}

//Bit operations
impl std::ops::BitOr for BitArray {
    type Output = BitArray;

    fn bitor(self, rhs: BitArray) -> BitArray {
        BitArray {
            bits: self.bits | rhs.bits,
        }
    }
}

impl std::ops::BitAnd for BitArray {
    type Output = BitArray;

    fn bitand(self, rhs: BitArray) -> BitArray {
        BitArray {
            bits: self.bits & rhs.bits,
        }
    }
}

impl std::ops::BitXor for BitArray {
    type Output = BitArray;

    fn bitxor(self, rhs: BitArray) -> BitArray {
        BitArray {
            bits: self.bits ^ rhs.bits,
        }
    }
}

impl std::ops::BitOrAssign for BitArray {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits;
    }
}

impl std::ops::BitXorAssign for BitArray {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.bits ^= rhs.bits;
    }
}

impl std::ops::Not for BitArray {
    type Output = BitArray;

    fn not(self) -> BitArray {
        BitArray { bits: !self.bits }
    }
}

impl std::ops::Shl<usize> for BitArray {
    type Output = BitArray;

    fn shl(self, rhs: usize) -> BitArray {
        BitArray {
            bits: self.bits << rhs,
        }
    }
}

impl std::ops::Shr<usize> for BitArray {
    type Output = BitArray;

    fn shr(self, rhs: usize) -> BitArray {
        BitArray {
            bits: self.bits >> rhs,
        }
    }
}


//Unit tests
#[cfg(test)]
mod bit_array_tests {
    use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
    use rand::Rng;

    #[test]
    fn translation_test() {
        let mut v = Vec::new();

        v.push(super::BitArray::empty());
        v.push(super::BitArray::full());

        let mut rng = ChaCha8Rng::seed_from_u64(0);

        for _ in 0..100 {
            let mut bb = super::BitArray::empty();

            for _ in 0..32 {
                let square = super::Square::from_usize(rng.gen_range(0..64));
                bb.set_bit(square);
            }

            v.push(bb);
        }

        for bb in v {
            for dx in -7..=7 {
                for dy in -7..=7 {
                    println!("Translation: ({}, {})", dx, dy);
                    bb.print();

                    let translated = bb.translate(dx, dy);

                    println!("Translated:");
                    translated.print();

                    
                    let mut expected = bb;
                    for _ in 0..dx {
                        expected = expected.right();
                    }
                    for _ in 0..-dx {
                        expected = expected.left();
                    }
                    for _ in 0..dy {
                        expected = expected.up();
                    }
                    for _ in 0..-dy {
                        expected = expected.down();
                    }

                    println!("Expected:");
                    expected.print();

                    assert_eq!(translated, expected, "Translation failed: ({}, {})", dx, dy);
                }
            }
        }
    }
}