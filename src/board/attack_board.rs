use rand::RngCore;

use super::{bit_array::BitArray, bit_array_lookup::ORTHOGONAL_MOVES, bit_board::{self, BitBoard}, dynamic_state::DynamicState, piece_type::ColoredPieceType};


pub struct FieldCounter {
    bits: [u64; 3],
}

impl FieldCounter {
    pub fn new() -> FieldCounter {
        FieldCounter {
            bits: [0; 3],
        }
    }

    pub fn increment(&mut self, mut value: u64) {
        let mut carry = value & self.bits[0];
        self.bits[0] ^= value;
        carry = carry & self.bits[1];
        self.bits[1] ^= carry;
        carry = carry & self.bits[2];
        self.bits[2] ^= carry;
    }

    pub fn decrement(&mut self, mut value: u64) {
        unimplemented!()
    }

    pub fn count(&self, index: i8) -> u64 {
        let mut value = (self.bits[0] >> index) & 1;
        value |= ((self.bits[1] >> index) & 1) << 1;
        value |= ((self.bits[2] >> index) & 1) << 2;

        return value;
    }
}

#[test] 
fn test_field_counter() {
    let mut counter = FieldCounter::new();
    let mut field = [0_64; 64];

    let mut rng = rand::thread_rng();
    for i in 0..8 {
        let bits = rng.next_u64();

        bits.print();

        counter.increment(bits);

        for j in 0..64 {
            let val = (bits >> j) & 1;
            if val >= 2 {
                println!("val: {}", val);
            }
            field[j] += val;
        }

        for j in 0..64 {
            assert_eq!(field[j], counter.count(j as i8));
        }
    }

}

// pub struct AttackBoard {
//     attack_counter: 
// }

// impl DynamicState for AttackBoard {
//     fn empty() -> Self {
//         AttackBoard {
//             pawn_attacks: 0,
//             knight_attacks: 0,
//             diagonal_attacks: 0,
//             orthogonal_attacks: 0,
//             king_attacks: 0,
//         }
//     }

//     fn add_piece(&mut self, pt: ColoredPieceType, s: i8, bit_board: &BitBoard) {
//         let piece_color = pt.color();
//         let piece_type = pt.piece_type();

//         //block slider
//         let orth_attacker = ORTHOGONAL_MOVES[s as usize] & bit_board.orthogonal_slider;

//     }
    
//     fn remove_piece(&mut self, pt: ColoredPieceType, s: i8) {
//         todo!()
//     }
// }