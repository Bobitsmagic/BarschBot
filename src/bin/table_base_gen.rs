use std::{
    collections::HashSet,
    io,
    sync::{Arc, Mutex},
    time::Instant,
};

use barschbot::{
    board::{
        bit_array::BitArray,
        piece_board::PieceBoard,
        piece_type::ColoredPieceType,
        player_color::PlayerColor::{self, Black},
        square,
    },
    moves::uci_move,
    tablebase::endgame::{
        board_generation,
        endgame_table::{self, EndGameTable},
    },
};
use bitintr::Blsic;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn count_boards_par(piece_count: usize) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(3)
        .build_global()
        .unwrap();

    let start_time = Instant::now();
    let table = EndGameTable::generate_par(piece_count);
    println!("Time: {:?}", start_time.elapsed());
    println!("Count: {}", table.count());
}

fn from_input() {
    println!("ExtraPieceCount?");
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer).unwrap();
    let piece_count: usize = buffer.trim().parse().unwrap();
    println!("ThreadCount?");
    buffer.clear();
    stdin.read_line(&mut buffer).unwrap();
    println!("Buffer: {buffer}");
    let thread_count: usize = buffer.trim().parse().unwrap();

    count_boards_par(piece_count);
}

fn main() {
    //2 piece: 125501770
    count_boards_par(3);

    // let mut vec = vec![1,2,3, 4];
    // vec.swap_remove(1);
    // dbg!(vec);
}
