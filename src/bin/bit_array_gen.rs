use std::{fs::File, io::{self, Write}};

use barschbot::board::{bit_array::{self, gen_rook_moves, BitArray}, bit_array_lookup, square::{Square, VALID_SQUARES}};

pub fn main() {
    // gen_bishop_move_table();
    // gen_rook_move_table();

    print_all_tables();
}



pub fn print_all_tables() {
    let pawn_moves = gen_pawn_move_masks();
    let knight_moves = gen_knight_move_mask();
    let king_moves = gen_king_move_mask();
    let diagonal_moves = gen_diagonal_move_mask();
    let orthogonal_moves = gen_orthogonal_move_mask();
    let queen_moves = gen_queen_move_mask();
    let rook_blocker_mask = gen_rook_blocker_mask();
    let bishop_blocker_mask = gen_bishop_blocker_mask();
    let rook_move_table = gen_rook_move_table();
    let bishop_move_table = gen_bishop_move_table();

    
    let mut s = String::new();
    s += &bit_array_to_string("PAWN_MOVES_WHITE", &pawn_moves.0);
    s += &bit_array_to_string("PAWN_MOVES_BLACK", &pawn_moves.1);
    s += &bit_array_to_string("KNIGHT_MOVES", &knight_moves);
    s += &bit_array_to_string("KING_MOVES", &king_moves);
    s += &bit_array_to_string("DIAGONAL_MOVES", &diagonal_moves);
    s += &bit_array_to_string("ORTHOGONAL_MOVES", &orthogonal_moves);
    s += &bit_array_to_string("QUEEN_MOVES", &queen_moves);
    s += &bit_array_to_string("ROOK_BLOCKER_MASK", &rook_blocker_mask);
    s += &bit_array_to_string("BISHOP_BLOCKER_MASK", &bishop_blocker_mask);
    s += &move_table_to_string("ROOK_MOVE_TABLE", &rook_move_table);
    s += &move_table_to_string("BISHOP_MOVE_TABLE", &bishop_move_table);

    //Write to text file
    let mut file = File::create("generated_files/lookup.rs").unwrap();
    file.write_all(s.as_bytes()).unwrap();
}

pub fn bit_array_to_string(name: &str, bit_array_array: &[BitArray]) -> String {
    let length = bit_array_array.len();
    
    let mut s = format!("pub const {}: [BitArray; {}] = [", name, length);

    for (i, bit_array) in bit_array_array.iter().enumerate() {
        s += &format!("\tBitArray {{ bits: 0x{:016x}}}", bit_array.bits);

        if i < length - 1 {
            s += &format!(",\n");
        } else {
            s += &format!("\n");
        }
    }

    s += &format!("];\n");

    return s;
}

pub fn move_table_to_string(name: &str, table: &[Vec<BitArray>]) -> String {
    let mut s = format!("pub const {}: [&[BitArray]; 64] = [", name);

    for (i, vec) in table.iter().enumerate() {
        s += &format!("\t&[");

        for (j, bit_array) in vec.iter().enumerate() {
            s += &format!("BitArray {{ bits: 0x{:016x}}}", bit_array.bits);

            if j < vec.len() - 1 {
                s += &format!(", ");
            }
        }

        s += &format!("]");

        if i < table.len() - 1 {
            s += &format!(",\n");
        } else {
            s += &format!("\n");
        }
    }

    s += &format!("];\n");

    return s;
}

pub fn gen_pawn_move_masks() -> ([BitArray; 64], [BitArray; 64]) {
    let mut white_moves = [BitArray::empty(); 64];
    let mut black_moves = [BitArray::empty(); 64];

    for s in VALID_SQUARES {
        let mut white_move = BitArray::empty();
        let mut black_move = BitArray::empty();

        white_move |= s.bit_array().up_left();
        white_move |= s.bit_array().up_right();

        black_move |= s.bit_array().down_left();
        black_move |= s.bit_array().down_right();

        white_moves[s as usize] = white_move;
        black_moves[s as usize] = black_move;
    }

    (white_moves, black_moves)
}

pub fn gen_knight_move_mask() -> [BitArray; 64] {
    let mut moves = [BitArray::empty(); 64];

    for s in VALID_SQUARES {
        let mut move_set = BitArray::empty();

        move_set |= s.bit_array().translate(2, 1);
        move_set |= s.bit_array().translate(2, -1);
        move_set |= s.bit_array().translate(-2, 1);
        move_set |= s.bit_array().translate(-2, -1);

        move_set |= s.bit_array().translate(1, 2);
        move_set |= s.bit_array().translate(1, -2);
        move_set |= s.bit_array().translate(-1, 2);
        move_set |= s.bit_array().translate(-1, -2);

        moves[s as usize] = move_set;
    }

    moves
}

pub fn gen_king_move_mask() -> [BitArray; 64] {
    let mut moves = [BitArray::empty(); 64];

    for s in VALID_SQUARES {
        let mut move_set = BitArray::empty();

        move_set |= s.bit_array().up();
        move_set |= s.bit_array().down();
        move_set |= s.bit_array().left();
        move_set |= s.bit_array().right();

        move_set |= s.bit_array().up_left();
        move_set |= s.bit_array().up_right();
        move_set |= s.bit_array().down_left();
        move_set |= s.bit_array().down_right();

        moves[s as usize] = move_set;
    }

    moves
}

pub fn gen_diagonal_move_mask() -> [BitArray; 64] {
    let mut moves = [BitArray::empty(); 64];

    for s in VALID_SQUARES {
        let mut move_set = BitArray::empty();

        for i in 1..8 {
            move_set |= s.bit_array().translate(i, i);
            move_set |= s.bit_array().translate(i, -i);
            move_set |= s.bit_array().translate(-i, i);
            move_set |= s.bit_array().translate(-i, -i);
        }

        moves[s as usize] = move_set;
    }

    moves
}

pub fn gen_orthogonal_move_mask() -> [BitArray; 64] {
    let mut moves = [BitArray::empty(); 64];

    for s in VALID_SQUARES {
        let mut move_set = BitArray::empty();

        for i in 1..8 {
            move_set |= s.bit_array().translate(i, 0);
            move_set |= s.bit_array().translate(-i, 0);
            move_set |= s.bit_array().translate(0, i);
            move_set |= s.bit_array().translate(0, -i);
        }

        moves[s as usize] = move_set;
    }

    moves
}

pub fn gen_queen_move_mask() -> [BitArray; 64] {
    let mut moves = [BitArray::empty(); 64];

    let orthogonal_moves = gen_orthogonal_move_mask();
    let diagonal_moves = gen_diagonal_move_mask();

    for s in VALID_SQUARES {
        moves[s as usize] = orthogonal_moves[s as usize] | diagonal_moves[s as usize];
    }

    moves
}

pub fn gen_rook_blocker_mask() -> [BitArray; 64] {
    let mut mask = [BitArray::empty(); 64];

    for s in VALID_SQUARES {
        let mut blocker_mask = BitArray::empty();

        let sx = s.file_index() as u8;
        let sy = s.rank_index() as u8;

        for i in 1..7 {
            let square = Square::from_rank_file_index(i, sy);
            blocker_mask.set_bit(square);

            let square = Square::from_rank_file_index(sx, i);
            blocker_mask.set_bit(square);
        }

        mask[s as usize] = blocker_mask;
    }

    mask
}

pub fn gen_bishop_blocker_mask() -> [BitArray; 64] {
    let moves = gen_diagonal_move_mask();

    let mut mask = [BitArray::empty(); 64];

    let boarder = bit_array_lookup::ROWS[0] | bit_array_lookup::ROWS[7] | bit_array_lookup::COLLUMNS[0] | bit_array_lookup::COLLUMNS[7];

    for s in VALID_SQUARES {
        mask[s as usize] = moves[s as usize] & !boarder;
    }

    mask
}

pub fn gen_rook_move_table() -> [Vec<BitArray>; 64] {
    let mut ret = [const { Vec::new() }; 64];

    let rook_blocker_mask = gen_rook_blocker_mask();
    for s in VALID_SQUARES {
        let mut move_set = Vec::new();

        let mask = rook_blocker_mask[s as usize];

        let bit_count = mask.count_bits();

        for index in 0..(1_u64 << bit_count) {
            let blocker = bitintr::Pdep::pdep(index, mask.bits);

            let moves = bit_array::gen_rook_moves(s, BitArray::empty(), BitArray::new(blocker));

            move_set.push(moves);
        }

        ret[s as usize] = move_set;
    }

    // println!("Final length: {}", ret.iter().map(|v| v.len()).sum::<usize>());
    // println!("Min length: {}", ret.iter().map(|v| v.len()).min().unwrap());
    // println!("Max length: {}", ret.iter().map(|v| v.len()).max().unwrap());

    ret
}

pub fn gen_bishop_move_table() -> [Vec<BitArray>; 64] {
    let mut ret = [const { Vec::new() }; 64];

    let bishop_blocker_mask = gen_bishop_blocker_mask();
    for s in VALID_SQUARES {
        let mut move_set = Vec::new();

        let mask = bishop_blocker_mask[s as usize];

        let bit_count = mask.count_bits();
        for index in 0..(1_u64 << bit_count) {
            let b_value = bitintr::Pdep::pdep(index, mask.bits);
            let blocker = BitArray::new(b_value);

            let moves = bit_array::gen_bishop_moves(s, BitArray::empty(), blocker);

            move_set.push(moves);
        }

        ret[s as usize] = move_set;
    }

    // println!("Final length: {}", ret.iter().map(|v| v.len()).sum::<usize>());
    // println!("Min length: {}", ret.iter().map(|v| v.len()).min().unwrap());
    // println!("Max length: {}", ret.iter().map(|v| v.len()).max().unwrap());

    ret
}