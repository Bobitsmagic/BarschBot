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
        return iterate_set_bits(self.bits);
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

    pub fn translate_vertical(&self, dy: i8) -> BitArray {
        debug_assert!(dy.abs() <= 7, "Invalid translation: ({}, {})", 0, dy);
        
        let shift_sum = dy * 8;
        if shift_sum > 0 {
            BitArray { bits: self.bits << shift_sum }
        }
        else {
            BitArray { bits: self.bits >> -shift_sum }
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
    
    pub fn is_full(&self) -> bool {
        self.bits == u64::MAX
    }
}

//Helper

pub fn iterate_set_bits(mut value: u64) -> impl Iterator<Item=u32> {
    return std::iter::from_fn(move || {
        if value != 0 {
            let index = value.trailing_zeros();
            value = bitintr::Blsr::blsr(value);
            // value &= value - 1;
            // value ^= 1_u64 << index;
            
            Some(index)
        }
        else {
            None
        }
    });
}


//Move generation
pub fn gen_rook_moves(square: Square, allied: BitArray, opponent: BitArray) -> BitArray {
    const DIRECTIONS: [(i8, i8); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut moves = BitArray::empty();
    let x = square.file_index() as i8;
    let y = square.rank_index() as i8;
    for (dx, dy) in DIRECTIONS {
        let mut sx = x + dx;
        let mut sy = y + dy;

        while sx >= 0 && sx < 8 && sy >= 0 && sy < 8 {
            let next = Square::from_rank_file_index(sy as u8, sx as u8);
            if opponent.get_bit(next) {
                moves.set_bit(next);
                break;
            }
            else if allied.get_bit(next) {
                break;
            }
            else {
                moves.set_bit(next);
            }

            sx += dx;
            sy += dy;
        }
    }

    return moves;
}

pub fn gen_bishop_moves(square: Square, allied: BitArray, opponent: BitArray) -> BitArray {  
    const DIRECTIONS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    let mut moves = BitArray::empty();
    let x = square.file_index() as i8;
    let y = square.rank_index() as i8;
    for (dx, dy) in DIRECTIONS {
        let mut sx = x + dx;
        let mut sy = y + dy;

        while sx >= 0 && sx < 8 && sy >= 0 && sy < 8 {
            let next = Square::from_rank_file_index(sy as u8, sx as u8);
            if opponent.get_bit(next) {
                moves.set_bit(next);
                break;
            }
            else if allied.get_bit(next) {
                break;
            }
            else {
                moves.set_bit(next);
            }

            sx += dx;
            sy += dy;
        }
    }

    return moves;
}

pub fn gen_queen_moves(square: Square, allied: BitArray, opponent: BitArray) -> BitArray {
    return gen_rook_moves(square, allied, opponent) | gen_bishop_moves(square, allied, opponent);
}

pub fn gen_rook_moves_pext(square: Square, occupied: BitArray) -> BitArray {
    let index = order_bits(occupied.bits, ROOK_BLOCKER_MASK[square as usize].bits);

    return ROOK_MOVE_TABLE[square as usize][index as usize];
}

pub fn gen_bishop_moves_pext(square: Square, occupied: BitArray) -> BitArray {
    let index = order_bits(occupied.bits, BISHOP_BLOCKER_MASK[square as usize].bits);

    return BISHOP_MOVE_TABLE[square as usize][index as usize];
}

pub fn order_bits(value: u64, mask: u64) -> u64 {
    return bitintr::Pext::pext(value, mask); //650 ms
    
    //return bitintr::Pdep::pdep(value, mask);
    // let mut ret = 0;
    // for i in iterate_set_bits(mask) {
    //     ret = (ret << 1) | (value >> i) & 1;        
    // }

    // return ret; //650 ms

    // unsafe {
    //     return core::arch::x86_64::_pext_u64(value, mask); //990 ms
    // }
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

impl std::ops::BitAndAssign for BitArray {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bits &= rhs.bits;
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
    use rand::{Rng, RngCore};

    use crate::board::{bit_array::{gen_bishop_moves, gen_bishop_moves_pext, BitArray}, square::{Square, VALID_SQUARES}};

    use super::{gen_rook_moves, gen_rook_moves_pext};

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

    #[test]
    fn bishop_move_gen_test() {
        let mut rng = ChaCha8Rng::seed_from_u64(0);

        for _ in 0..100 {
            
            let mut allied = BitArray::empty();
            let mut opponent = BitArray::empty();

            for x in 0..8 {
                for y in 0..8 {
                    let square = Square::from_rank_file_index(y, x);
                    if rng.gen_bool(0.1) {
                        allied.set_bit(square);
                    } else if rng.gen_bool(0.1) {
                        opponent.set_bit(square);
                    }
                }
            }
            
            for s in VALID_SQUARES {
                let m1 = gen_bishop_moves(s, allied, opponent);
                let m2 = gen_bishop_moves_pext(s, allied | opponent) & !allied;
                
                if m1 != m2 {
                    println!("Allied:");
                    allied.print();
                    println!("Opponent:");
                    opponent.print();
                    println!("Square: {}", s.to_string());
                    println!("gen_moves:");
                    m1.print();
                    println!("gen_moves_pext:");
                    m2.print();
                    panic!();
                }                
            }
        }
    }

    #[test]
    fn rook_move_gen_test() {
        let mut rng = ChaCha8Rng::seed_from_u64(0);

        for _ in 0..100 {
            
            let mut allied = BitArray::empty();
            let mut opponent = BitArray::empty();

            for x in 0..8 {
                for y in 0..8 {
                    let square = Square::from_rank_file_index(y, x);
                    if rng.gen_bool(0.1) {
                        allied.set_bit(square);
                    } else if rng.gen_bool(0.1) {
                        opponent.set_bit(square);
                    }
                }
            }
            
            for s in VALID_SQUARES {
                let m1 = gen_rook_moves(s, allied, opponent);
                let m2 = gen_rook_moves_pext(s, allied | opponent) & !allied;
                
                if m1 != m2 {
                    println!("Allied:");
                    allied.print();
                    println!("Opponent:");
                    opponent.print();
                    println!("Square: {}", s.to_string());
                    println!("gen_moves:");
                    m1.print();
                    println!("gen_moves_pext:");
                    m2.print();
                    panic!();
                }                
            }
        }
    }
}
