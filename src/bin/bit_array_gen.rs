use barschbot::board::{bit_array::BitArray, square::VALID_SQUARES};

pub fn main() {
    print_all_tables();
}



pub fn print_all_tables() {
    let pawn_moves = gen_pawn_moves();
    let knight_moves = gen_knight_moves();
    let king_moves = gen_king_moves();
    let diagonal_moves = gen_diagonal_moves();
    let orthogonal_moves = gen_orthogonal_moves();
    let queen_moves = gen_queen_moves();

    print_bit_array_array("PAWN_MOVES_WHITE", &pawn_moves.0);
    print_bit_array_array("PAWN_MOVES_BLACK", &pawn_moves.1);
    print_bit_array_array("KNIGHT_MOVES", &knight_moves);
    print_bit_array_array("KING_MOVES", &king_moves);
    print_bit_array_array("DIAGONAL_MOVES", &diagonal_moves);
    print_bit_array_array("ORTHOGONAL_MOVES", &orthogonal_moves);
    print_bit_array_array("QUEEN_MOVES", &queen_moves);
}

pub fn print_bit_array_array(name: &str, bit_array_array: &[BitArray]) {
    let length = bit_array_array.len();
    println!("const {}: [BitArray; {}] = [", name, length);

    for (i, bit_array) in bit_array_array.iter().enumerate() {
        print!("\tBitArray {{ bits: 0x{:016x}}}", bit_array.bits);

        if i < length - 1 {
            println!(",");
        } else {
            println!();
        }
    }

    println!("];");
}

pub fn gen_pawn_moves() -> ([BitArray; 64], [BitArray; 64]) {
    let mut white_moves = [BitArray::empty(); 64];
    let mut black_moves = [BitArray::empty(); 64];

    for s in VALID_SQUARES {
        let mut white_move = BitArray::empty();
        let mut black_move = BitArray::empty();

        white_move |= s.get_bitarray().up_left();
        white_move |= s.get_bitarray().up_right();

        black_move |= s.get_bitarray().down_left();
        black_move |= s.get_bitarray().down_right();

        white_moves[s as usize] = white_move;
        black_moves[s as usize] = black_move;
    }

    (white_moves, black_moves)
}

pub fn gen_knight_moves() -> [BitArray; 64] {
    let mut moves = [BitArray::empty(); 64];

    for s in VALID_SQUARES {
        let mut move_set = BitArray::empty();

        move_set |= s.get_bitarray().translate(2, 1);
        move_set |= s.get_bitarray().translate(2, -1);
        move_set |= s.get_bitarray().translate(-2, 1);
        move_set |= s.get_bitarray().translate(-2, -1);

        move_set |= s.get_bitarray().translate(1, 2);
        move_set |= s.get_bitarray().translate(1, -2);
        move_set |= s.get_bitarray().translate(-1, 2);
        move_set |= s.get_bitarray().translate(-1, -2);

        moves[s as usize] = move_set;
    }

    moves
}

pub fn gen_king_moves() -> [BitArray; 64] {
    let mut moves = [BitArray::empty(); 64];

    for s in VALID_SQUARES {
        let mut move_set = BitArray::empty();

        move_set |= s.get_bitarray().up();
        move_set |= s.get_bitarray().down();
        move_set |= s.get_bitarray().left();
        move_set |= s.get_bitarray().right();

        move_set |= s.get_bitarray().up_left();
        move_set |= s.get_bitarray().up_right();
        move_set |= s.get_bitarray().down_left();
        move_set |= s.get_bitarray().down_right();

        moves[s as usize] = move_set;
    }

    moves
}

pub fn gen_diagonal_moves() -> [BitArray; 64] {
    let mut moves = [BitArray::empty(); 64];

    for s in VALID_SQUARES {
        let mut move_set = BitArray::empty();

        for i in 1..8 {
            move_set |= s.get_bitarray().translate(i, i);
            move_set |= s.get_bitarray().translate(i, -i);
            move_set |= s.get_bitarray().translate(-i, i);
            move_set |= s.get_bitarray().translate(-i, -i);
        }

        moves[s as usize] = move_set;
    }

    moves
}

pub fn gen_orthogonal_moves() -> [BitArray; 64] {
    let mut moves = [BitArray::empty(); 64];

    for s in VALID_SQUARES {
        let mut move_set = BitArray::empty();

        for i in 1..8 {
            move_set |= s.get_bitarray().translate(i, 0);
            move_set |= s.get_bitarray().translate(-i, 0);
            move_set |= s.get_bitarray().translate(0, i);
            move_set |= s.get_bitarray().translate(0, -i);
        }

        moves[s as usize] = move_set;
    }

    moves
}

pub fn gen_queen_moves() -> [BitArray; 64] {
    let mut moves = [BitArray::empty(); 64];

    let orthogonal_moves = gen_orthogonal_moves();
    let diagonal_moves = gen_diagonal_moves();

    for s in VALID_SQUARES {
        moves[s as usize] = orthogonal_moves[s as usize] | diagonal_moves[s as usize];
    }

    moves
}