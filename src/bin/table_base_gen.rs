use std::{
    collections::HashSet,
    io,
    sync::{Arc, Mutex},
    time::Instant,
};

use barschbot::{
    board::{bit_array::BitArray, piece_board::PieceBoard, piece_type::ColoredPieceType, player_color::PlayerColor::{self, Black}, square}, moves::uci_move, tablebase::{
        self, endgame_table::{
            self, count_boards, count_boards_sym, count_boards_sym_tiny, generate_game_states_sym_king_boards, generate_king_boards, generate_king_boards_sym, generate_king_boards_sym_color,
        }, endgame_type, tiny_state::TinyState,
    },
};
use bitintr::Blsic;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn count_boards_par(piece_count: usize, thread_count: usize) {
    let mut start_time = Instant::now();
    let tny_set = count_boards_sym_tiny(piece_count);
    println!("Tny time: {:?}", start_time.elapsed());
    println!("Tny count: {}", tny_set.len());
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

    count_boards_par(piece_count, thread_count);
}

fn main() {
    //2 piece: 125501770
    count_boards_par(4, 4);
}
