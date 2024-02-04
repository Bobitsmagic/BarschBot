use core::panic;
use std::time::Instant;

use crate::{bitboard_helper, square::{self, Square}};

    
pub fn benchmark() {
    for _ in 0..5 {
        let start = Instant::now();
        let res = fill_benchmark();
        let duration = start.elapsed();
        println!("Result: {}", res);
        println!("Time: {:?}", duration);
    }
}

pub fn fill_benchmark() -> u32 {
    let mut sum = 0;
    for i in 0..1_000_000 {
        for sq in square::ARRAY.iter().take(64) {

            let allied = (i * 123456789) & bitboard_helper::DIAGONAL_ATTACKS[*sq as usize] | sq.bit_board();
            let opponent = (i * 987654321) & bitboard_helper::DIAGONAL_ATTACKS[*sq as usize] & !allied;
            let v1 = bitboard_helper::gen_rook_moves(*sq, allied, opponent);
            
            sum += v1.count_ones();
            
             /* 
            let v2 = bitboard_helper::fill_orthogonal_2(*sq, allied, opponent);
            if v1 != v2 {
                println!("Error at square: {}", sq.to_string());
                println!("Allied:");

                bitboard_helper::print_bitboard(allied);

                println!("Opponent:");

                bitboard_helper::print_bitboard(opponent);

                println!("Values:  ");

                bitboard_helper::print_bitboard(v1);
                bitboard_helper::print_bitboard(v2);
                panic!("Error");
            }
            */
        }
    }

    return sum;
}