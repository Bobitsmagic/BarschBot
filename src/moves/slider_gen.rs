use rand_chacha::ChaCha8Rng;

use crate::board::{bit_array::BitArray, bit_array_lookup::{BISHOP_BLOCKER_MASK, BISHOP_MOVE_TABLE, ROOK_BLOCKER_MASK, ROOK_MOVE_TABLE}, perfect_hashing, square::{Square, VALID_SQUARES}};

use super::kogge_gen::{DIAGONAL_FILL_FUNCTIONS_CAP, ORTHOGONAL_FILL_FUNCTIONS_CAP};

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

pub fn gen_rook_phf(square: Square, occupied: u64) -> u64 {
    let mask = ROOK_BLOCKER_MASK[square as usize];
    let bits = mask & occupied;

    *perfect_hashing::ROOK_TABLE[square as usize].get(&bits).unwrap()
}

pub fn gen_bishop_phf(square: Square, occupied: u64) -> u64 {
    let mask = BISHOP_BLOCKER_MASK[square as usize];
    let bits = mask & occupied;

    *perfect_hashing::BISHOP_TABLE[square as usize].get(&bits).unwrap()
}

pub fn gen_rook_moves_kogge(bb: u64, allied: u64, opponent: u64) -> u64 {
    let mut next = 0;
    let free = !(allied | opponent);

    for f in ORTHOGONAL_FILL_FUNCTIONS_CAP {
        next |= f(bb, free);
    }

    return next & !allied;
}

pub fn gen_bishop_moves_kogge(bb: u64, allied: u64, opponent: u64) -> u64 {
    let mut next = 0;
    let free = !(allied | opponent);

    for f in DIAGONAL_FILL_FUNCTIONS_CAP {
        next |= f(bb, free);
    }

    return next & !allied;
}

#[cfg(test)]
mod slider_gen_test {
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    use crate::board::{bit_array::BitArray, square::{Square, VALID_SQUARES}};

    use super::{gen_bishop_moves, gen_bishop_moves_kogge, gen_bishop_moves_pext, gen_bishop_phf, gen_rook_moves, gen_rook_moves_kogge, gen_rook_moves_pext, gen_rook_phf};

    fn fill_board(rng: &mut ChaCha8Rng) -> (u64, u64) {
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
    
        return (allied, opponent);
    
    }

    #[test]
    fn all_slider_test() {
        let mut rng = ChaCha8Rng::seed_from_u64(0);
    
        for _ in 0..1000 {
            
            let (allied, opponent) = fill_board(&mut rng);
            
            for s in VALID_SQUARES {
                let allied = allied | s.bit_array();

                let m1 = gen_bishop_moves(s, allied, opponent);
                let m2 = gen_bishop_moves_pext(s, allied | opponent) & !allied;
                let m3 = gen_bishop_phf(s, allied | opponent) & !allied;
                let m4 = gen_bishop_moves_kogge(s.bit_array(), allied, opponent);
                
                if m1 != m2 || m1 != m3 || m1 != m4 {
                    println!("Allied:");
                    allied.print();
                    println!("Opponent:");
                    opponent.print();
                    println!("Square: {}", s.to_string());
                    println!("gen_moves:");
                    m1.print();
                    println!("gen_moves_pext:");
                    m2.print();
                    println!("gen_moves_phf:");
                    m3.print();
                    println!("gen_moves_kogge:");
                    m4.print();
                    panic!();
                }

                let m1 = gen_rook_moves(s, allied, opponent);
                let m2 = gen_rook_moves_pext(s, allied | opponent) & !allied;
                let m3 = gen_rook_phf(s, allied | opponent) & !allied;
                let m4 = gen_rook_moves_kogge(s.bit_array(), allied, opponent);
                
                if m1 != m2 || m1 != m3 || m1 != m4 {
                    println!("Allied:");
                    allied.print();
                    println!("Opponent:");
                    opponent.print();
                    println!("Square: {}", s.to_string());
                    println!("gen_moves:");
                    m1.print();
                    println!("gen_moves_pext:");
                    m2.print();
                    println!("gen_moves_phf:");
                    m3.print();
                    println!("gen_moves_kogge:");
                    m4.print();
                    panic!();
                }
            }
        }
    }
}