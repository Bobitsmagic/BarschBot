use super::square::Square;

//Constants
pub const ROWS: [BitArray; 8] = [
    BitArray { bits: 0xFF },
    BitArray { bits: 0xFF00 },
    BitArray { bits: 0xFF0000 },
    BitArray { bits: 0xFF000000 },
    BitArray { bits: 0xFF00000000 },
    BitArray { bits: 0xFF0000000000 },
    BitArray { bits: 0xFF000000000000 },
    BitArray { bits: 0xFF00000000000000 },
];

pub const COLLUMNS: [BitArray; 8] = [
    BitArray { bits: 0x0101010101010101 },
    BitArray { bits: 0x0202020202020202 },
    BitArray { bits: 0x0404040404040404 },
    BitArray { bits: 0x0808080808080808 },
    BitArray { bits: 0x1010101010101010 },
    BitArray { bits: 0x2020202020202020 },
    BitArray { bits: 0x4040404040404040 },
    BitArray { bits: 0x8080808080808080 },
];

pub const SQUARES: [BitArray; 65] = [
    BitArray { bits: 0x1 },
    BitArray { bits: 0x2 },
    BitArray { bits: 0x4 },
    BitArray { bits: 0x8 },
    BitArray { bits: 0x10 },
    BitArray { bits: 0x20 },
    BitArray { bits: 0x40 },
    BitArray { bits: 0x80 },
    BitArray { bits: 0x100 },
    BitArray { bits: 0x200 },
    BitArray { bits: 0x400 },
    BitArray { bits: 0x800 },
    BitArray { bits: 0x1000 },
    BitArray { bits: 0x2000 },
    BitArray { bits: 0x4000 },
    BitArray { bits: 0x8000 },
    BitArray { bits: 0x10000 },
    BitArray { bits: 0x20000 },
    BitArray { bits: 0x40000 },
    BitArray { bits: 0x80000 },
    BitArray { bits: 0x100000 },
    BitArray { bits: 0x200000 },
    BitArray { bits: 0x400000 },
    BitArray { bits: 0x800000 },
    BitArray { bits: 0x1000000 },
    BitArray { bits: 0x2000000 },
    BitArray { bits: 0x4000000 },
    BitArray { bits: 0x8000000 },
    BitArray { bits: 0x10000000 },
    BitArray { bits: 0x20000000 },
    BitArray { bits: 0x40000000 },
    BitArray { bits: 0x80000000 },
    BitArray { bits: 0x100000000 },
    BitArray { bits: 0x200000000 },
    BitArray { bits: 0x400000000 },
    BitArray { bits: 0x800000000 },
    BitArray { bits: 0x1000000000 },
    BitArray { bits: 0x2000000000 },
    BitArray { bits: 0x4000000000 },
    BitArray { bits: 0x8000000000 },
    BitArray { bits: 0x10000000000 },
    BitArray { bits: 0x20000000000 },
    BitArray { bits: 0x40000000000 },
    BitArray { bits: 0x80000000000 },
    BitArray { bits: 0x100000000000 },
    BitArray { bits: 0x200000000000 },
    BitArray { bits: 0x400000000000 },
    BitArray { bits: 0x800000000000 },
    BitArray { bits: 0x1000000000000 },
    BitArray { bits: 0x2000000000000 },
    BitArray { bits: 0x4000000000000 },
    BitArray { bits: 0x8000000000000 },
    BitArray { bits: 0x10000000000000 },
    BitArray { bits: 0x20000000000000 },
    BitArray { bits: 0x40000000000000 },
    BitArray { bits: 0x80000000000000 },
    BitArray { bits: 0x100000000000000 },
    BitArray { bits: 0x200000000000000 },
    BitArray { bits: 0x400000000000000 },
    BitArray { bits: 0x800000000000000 },
    BitArray { bits: 0x1000000000000000 },
    BitArray { bits: 0x2000000000000000 },
    BitArray { bits: 0x4000000000000000 },
    BitArray { bits: 0x8000000000000000 },
    BitArray { bits: 0x0 },
];

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BitArray {
    pub bits: u64,
}

impl BitArray {
    //Constructor
    pub fn empty() -> BitArray {
        BitArray { bits: 0 }
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