use std::{fs::File, io::Write};

use barschbot::board::{bit_array::{self, order_bits, BitArray}, bit_array_lookup::{self, SQUARES}, square::{Square, VALID_SQUARES}};

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
    s += &in_between_table_to_string();

    //Write to text file
    let mut file = File::create("generated_files/lookup.rs").unwrap();
    file.write_all(s.as_bytes()).unwrap();
}

pub fn bit_array_to_string(name: &str, bit_array_array: &[u64]) -> String {
    let length = bit_array_array.len();
    
    let mut s = format!("pub const {}: [u64; {}] = [", name, length);

    for (i, bit_array) in bit_array_array.iter().enumerate() {
        s += &format!("\tu64 {{ bits: 0x{:016x}}}", bit_array);

        if i < length - 1 {
            s += &format!(",\n");
        } else {
            s += &format!("\n");
        }
    }

    s += &format!("];\n");

    return s;
}

pub fn move_table_to_string(name: &str, table: &[Vec<u64>]) -> String {
    let mut s = format!("pub const {}: [&[u64]; 64] = [", name);

    for (i, vec) in table.iter().enumerate() {
        s += &format!("\t&[");

        for (j, bit_array) in vec.iter().enumerate() {
            s += &format!("u64 {{ bits: 0x{:016x}}}", bit_array);

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

pub fn in_between_table_to_string() -> String{
    let table = gen_in_between_table();

    let mut s = String::new();
    s += "pub const IN_BETWEEN_TABLE: [[u64; 64]; 64] = [\n";

    for (i, row) in table.iter().enumerate() {
        s += "\t[";

        for (j, bit_array) in row.iter().enumerate() {
            s += &format!("u64 {{ bits: 0x{:016x}}}", bit_array);

            if j < row.len() - 1 {
                s += ", ";
            }
        }

        s += "]";

        if i < table.len() - 1 {
            s += ",\n";
        } else {
            s += "\n";
        }
    }

    s += "];\n";

    return s;
}

pub fn gen_pawn_move_masks() -> ([u64; 64], [u64; 64]) {
    let mut white_moves = [0; 64];
    let mut black_moves = [0; 64];

    for s in VALID_SQUARES {
        let mut white_move = 0;
        let mut black_move = 0;

        white_move |= s.bit_array().up().left();
        white_move |= s.bit_array().up().right();

        black_move |= s.bit_array().down().left();
        black_move |= s.bit_array().down().right();

        white_moves[s as usize] = white_move;
        black_moves[s as usize] = black_move;
    }

    (white_moves, black_moves)
}

pub fn gen_knight_move_mask() -> [u64; 64] {
    let mut moves = [0; 64];

    for s in VALID_SQUARES {
        let mut move_set = 0;

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

pub fn gen_king_move_mask() -> [u64; 64] {
    let mut moves = [0; 64];

    for s in VALID_SQUARES {
        let mut move_set = 0;

        move_set |= s.bit_array().up();
        move_set |= s.bit_array().down();
        move_set |= s.bit_array().left();
        move_set |= s.bit_array().right();

        move_set |= s.bit_array().up().left();
        move_set |= s.bit_array().up().right();
        move_set |= s.bit_array().down().left();
        move_set |= s.bit_array().down().right();

        moves[s as usize] = move_set;
    }

    moves
}

pub fn gen_diagonal_move_mask() -> [u64; 64] {
    let mut moves = [0; 64];

    for s in VALID_SQUARES {
        let mut move_set = 0;

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

pub fn gen_orthogonal_move_mask() -> [u64; 64] {
    let mut moves = [0; 64];

    for s in VALID_SQUARES {
        let mut move_set = 0;

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

pub fn gen_queen_move_mask() -> [u64; 64] {
    let mut moves = [0; 64];

    let orthogonal_moves = gen_orthogonal_move_mask();
    let diagonal_moves = gen_diagonal_move_mask();

    for s in VALID_SQUARES {
        moves[s as usize] = orthogonal_moves[s as usize] | diagonal_moves[s as usize];
    }

    moves
}

pub fn gen_rook_blocker_mask() -> [u64; 64] {
    let mut mask = [0; 64];

    for s in VALID_SQUARES {
        let mut blocker_mask = 0;

        let sx = s.file_index() as u8;
        let sy = s.rank_index() as u8;

        for i in 1..7 {
            let square = Square::from_rank_file_index(sy, i);
            blocker_mask.set_bit(square);

            let square = Square::from_rank_file_index(i, sx);
            blocker_mask.set_bit(square);
        }

        mask[s as usize] = blocker_mask;
    }

    mask
}

pub fn gen_bishop_blocker_mask() -> [u64; 64] {
    let moves = gen_diagonal_move_mask();

    let mut mask = [0; 64];

    let boarder = bit_array_lookup::ROWS[0] | bit_array_lookup::ROWS[7] | bit_array_lookup::COLLUMNS[0] | bit_array_lookup::COLLUMNS[7];

    for s in VALID_SQUARES {
        mask[s as usize] = moves[s as usize] & !boarder;
    }

    mask
}

pub fn gen_rook_move_table() -> [Vec<u64>; 64] {
    let mut ret = [const { Vec::new() }; 64];

    let rook_blocker_mask = gen_rook_blocker_mask();
    for s in VALID_SQUARES {
        let mut move_set = Vec::new();

        let mask = rook_blocker_mask[s as usize];

        // println!("Mask");
        // mask.print();

        let bit_count = mask.count_ones();

        for index in 0..(1_u64 << bit_count) {
            let blocker = bitintr::Pdep::pdep(index, mask);

            let idx = order_bits(blocker, mask);
            
            assert_eq!(idx, index);
            
            let moves = bit_array::gen_rook_moves(s, 0, blocker);
            
            // println!("Blocker: ");
            // (blocker).print();
            // moves.print();

            move_set.push(moves);
        }

        ret[s as usize] = move_set;

        // panic!();
    }

    // println!("Final length: {}", ret.iter().map(|v| v.len()).sum::<usize>());
    // println!("Min length: {}", ret.iter().map(|v| v.len()).min().unwrap());
    // println!("Max length: {}", ret.iter().map(|v| v.len()).max().unwrap());

    ret
}

pub fn gen_bishop_move_table() -> [Vec<u64>; 64] {
    let mut ret = [const { Vec::new() }; 64];

    let bishop_blocker_mask = gen_bishop_blocker_mask();
    for s in VALID_SQUARES {
        let mut move_set = Vec::new();

        let mask = bishop_blocker_mask[s as usize];

        let bit_count = mask.count_ones();
        for index in 0..(1_u64 << bit_count) {
            let blocker = bitintr::Pdep::pdep(index, mask);
            

            let idx = order_bits(blocker, mask);

            assert_eq!(idx, index);

            let moves = bit_array::gen_bishop_moves(s, 0, (blocker));

            move_set.push(moves);
        }

        ret[s as usize] = move_set;
    }

    // println!("Final length: {}", ret.iter().map(|v| v.len()).sum::<usize>());
    // println!("Min length: {}", ret.iter().map(|v| v.len()).min().unwrap());
    // println!("Max length: {}", ret.iter().map(|v| v.len()).max().unwrap());

    ret
}

pub fn gen_in_between_table() -> [[u64; 64]; 64] {
    let mut result = [[0; 64]; 64];

    for x1 in 0_i8..8 {
        for y1 in 0_i8..8 {
            for x2 in 0_i8..8 {
                for y2 in 0_i8..8 {
                    let dx = x2 - x1;
                    let dy = y2 - y1;

                    //Orhtogonal or diagonal
                    if dx == 0 || dy == 0 || dx.abs() == dy.abs() {
                        let dx = dx.signum();
                        let dy = dy.signum();

                        let mut in_between = 0;
                        let mut x = x1 + dx;
                        let mut y = y1 + dy;


                        while x != x2 || y != y2 {
                            in_between.set_bit(Square::from_rank_file_index(y as u8, x as u8));

                            x += dx.signum();
                            y += dy.signum();
                        }

                        result[(x1 + y1 * 8) as usize][(x2 + y2 * 8) as usize] = in_between;
                    }
                }
            }
        }
    }
    
    return result;
}