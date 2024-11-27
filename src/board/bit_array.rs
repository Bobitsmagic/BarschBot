
use crate::board::square::VALID_SQUARES;

use super::{bit_array_lookup::*, square::Square};

pub trait BitArray {
    fn set_bit(&mut self, square: Square);
    fn clear_bit(&mut self, square: Square);
    fn get_bit(self, square: Square) -> bool;
    fn flip_bit(&mut self, square: Square);

    fn translate(self, dx: i8, dy: i8) -> Self;
    fn translate_vertical(self, dy: i8) -> Self;
    fn left(self) -> Self;
    fn right(self) -> Self;
    fn up(self) -> Self;
    fn down(self) -> Self;

    fn iterate_set_bit_fields(self) -> impl Iterator<Item=u64>;
    fn iterate_set_bits_indices(self) -> impl Iterator<Item=u32>;
    fn iterate_squares(self) -> impl Iterator<Item=Square>;

    fn to_square(self) -> Square;

    fn print(self);
}

impl BitArray for u64 {
    fn set_bit(&mut self, square: Square) {
        *self |= 1 << square as u8;
    }
    fn clear_bit(&mut self, square: Square) {
        *self &= !(1 << square as u8);
    }
    fn flip_bit(&mut self, square: Square) {
        *self ^= 1 << square as u8;
    }

    fn get_bit(self, square: Square) -> bool {
        (self & (1 << square as u8)) != 0
    }

    fn to_square(self) -> Square {
        debug_assert!(self.count_ones() == 1, "Invalid bitboard: {}", self);
        VALID_SQUARES[self.trailing_zeros() as usize]
    }

    fn translate(self, dx: i8, dy: i8) -> u64 {
        debug_assert!(dx.abs() <= 7 && dy.abs() <= 7, "Invalid translation: ({}, {})", dx, dy);
        
        let mask = if dx >= 0 { ACCUM_COLLUMNS[(7 - dx) as usize] } else { !ACCUM_COLLUMNS[(-dx) as usize - 1] };

        let shift_sum = dx + dy * 8;
        if shift_sum > 0 {
            (self & mask) << shift_sum
        }
        else {
            (self & mask) >> -shift_sum
        }
    }

    fn translate_vertical(self, dy: i8) -> u64 {
        let shift_sum = dy * 8;
        if shift_sum > 0 {
            self << shift_sum
        }
        else {
            self >> -shift_sum
        }
    }

    fn left(self) -> u64 {
        (self & !ACCUM_COLLUMNS[0]) >> 1
    }
    fn right(self) -> u64 {
        (self << 1) & !ACCUM_COLLUMNS[0]
    }
    fn up(self) -> u64 {
        self << 8
    }
    fn down(self) -> u64 {
        self >> 8
    }

    fn print(self) {
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

    fn iterate_set_bit_fields(mut self) -> impl Iterator<Item=u64> {
        return std::iter::from_fn(move || {
            if self != 0 {
                let value = self & (!self + 1);
                self = bitintr::Blsr::blsr(self);
                // value &= value - 1;
                // value ^= 1_u64 << index;
                
                Some(value)
            }
            else {
                None
            }
        });
    }

    fn iterate_set_bits_indices(mut self) -> impl Iterator<Item=u32> {
        return std::iter::from_fn(move || {
            if self != 0 {
                let index = self.trailing_zeros();
                self = bitintr::Blsr::blsr(self);
                // value &= value - 1;
                // value ^= 1_u64 << index;
                
                Some(index)
            }
            else {
                None
            }
        });
    }
    
    fn iterate_squares(self) -> impl Iterator<Item=Square> {
        self.iterate_set_bits_indices().map(|v| Square::from_usize(v as usize))
    }
}



//Move generation
pub fn gen_rook_moves(square: Square, allied: u64, opponent: u64) -> u64 {
    const DIRECTIONS: [(i8, i8); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut moves = 0;
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

pub fn gen_bishop_moves(square: Square, allied: u64, opponent: u64) -> u64 {  
    const DIRECTIONS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    let mut moves = 0;
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

pub fn gen_queen_moves(square: Square, allied: u64, opponent: u64) -> u64 {
    return gen_rook_moves(square, allied, opponent) | gen_bishop_moves(square, allied, opponent);
}

pub fn gen_rook_moves_pext(square: Square, occupied: u64) -> u64 {
    let index = order_bits(occupied, ROOK_BLOCKER_MASK[square as usize]);

    return ROOK_MOVE_TABLE[square as usize][index as usize];
}

pub fn gen_bishop_moves_pext(square: Square, occupied: u64) -> u64 {
    let index = order_bits(occupied, BISHOP_BLOCKER_MASK[square as usize]);

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

//Unit tests
#[cfg(test)]
mod bit_array_tests {
    use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
    use rand::Rng;

    use crate::board::{bit_array::{gen_bishop_moves, gen_bishop_moves_pext}, square::{Square, VALID_SQUARES}};

    use super::{gen_rook_moves, gen_rook_moves_pext, BitArray};

    #[test]
    fn translation_test() {
        let mut v = Vec::new();

        v.push(0);
        v.push(u64::MAX);

        let mut rng = ChaCha8Rng::seed_from_u64(0);

        for _ in 0..100 {
            let mut bb = 0;

            for _ in 0..32 {
                let square = super::Square::from_usize(rng.gen_range(0..64));
                bb.set_bit(square);
            }

            v.push(bb);
        }

        for bb in v {
            for dx in -7..=7 {
                for dy in -7..=7 {
                    // println!("Translation: ({}, {})", dx, dy);
                    // bb.print();

                    let translated = bb.translate(dx, dy);

                    // println!("Translated:");
                    // translated.print();

                    
                    let mut expected = bb;
                    for _ in 0..dx {
                        expected = expected.right();
                        // println!("Right");
                    }
                    for _ in 0..-dx {
                        expected = expected.left();
                        // println!("Left");
                        // expected.print();
                    }
                    for _ in 0..dy {
                        expected = expected.up();
                        // println!("Up");
                    }
                    for _ in 0..-dy {
                        expected = expected.down();
                        // println!("Down");
                    }

                    // println!("Expected:");
                    // expected.print();

                    assert_eq!(translated, expected, "Translation failed: ({}, {})", dx, dy);
                }
            }
        }
    }

    #[test]
    fn bishop_move_gen_test() {
        let mut rng = ChaCha8Rng::seed_from_u64(0);

        for _ in 0..100 {
            
            let mut allied = 0;
            let mut opponent = 0;

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
            
            let mut allied = 0;
            let mut opponent = 0;

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
