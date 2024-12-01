use super::{bit_array_lookup::*};
use crate::board::square::{self, Square};

pub trait BitArray {
    fn set_bit(&mut self, i8: i8);
    fn clear_bit(&mut self, i8: i8);
    fn get_bit(self, i8: i8) -> bool;
    fn flip_bit(&mut self, i8: i8);

    fn translate(self, dx: i8, dy: i8) -> Self;
    fn translate_vertical(self, dy: i8) -> Self;
    fn left(self) -> Self;
    fn right(self) -> Self;
    fn up(self) -> Self;
    fn down(self) -> Self;

    fn iterate_set_bit_fields(self) -> impl Iterator<Item=u64>;
    fn iterate_set_bits_indices(self) -> impl Iterator<Item=u32>;
    fn iterate_squares(self) -> impl Iterator<Item=i8>;

    fn lowest_square_index(self) -> u32;

    fn print(self);
}

impl BitArray for u64 {
    fn set_bit(&mut self, i8: i8) {
        *self |= 1 << i8;
    }

    fn clear_bit(&mut self, i8: i8) {
        *self &= !(1 << i8);
    }
    fn flip_bit(&mut self, i8: i8) {
        *self ^= 1 << i8;
    }

    fn get_bit(self, i8: i8) -> bool {
        (self & (1 << i8)) != 0
    }

    fn lowest_square_index(self) -> u32 {
        debug_assert!(self.count_ones() == 1, "Invalid bitboard: {}", self);
        self.trailing_zeros()
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
                let i8 = square::from_file_rank(file, rank);
                s += &format!("{} ", if self.get_bit(i8) { "■" } else { "□" });
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
    
    fn iterate_squares(self) -> impl Iterator<Item=i8> {
        self.iterate_set_bits_indices().map(|v| v as i8)
    }
}

//Unit tests
#[cfg(test)]
mod bit_array_tests {
    use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
    use rand::Rng;

    use super::BitArray;

    #[test]
    fn translation_test() {
        let mut v = Vec::new();

        v.push(0);
        v.push(u64::MAX);

        let mut rng = ChaCha8Rng::seed_from_u64(0);

        for _ in 0..100 {
            let mut bb = 0;

            for _ in 0..32 {
                let i8 = rng.gen_range(0_i8..64);
                bb.set_bit(i8);
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
}
