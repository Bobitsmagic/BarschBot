use std::{collections::HashMap, fs::File, io::Write};

use barschbot::{board::{bit_array_lookup::{BISHOP_BLOCKER_MASK, ROOK_BLOCKER_MASK}, square::VALID_SQUARES}, moves::slider_gen::{gen_bishop_moves, gen_rook_moves}};

fn main() {
    let mut s = String::new();

    s += "use phf::{phf_map, Map};\n\n";

    let rook_table = gen_rook_move_table();

    s += &table_string(&rook_table, "ROOK_TABLE");

    let bishop_table = gen_bishop_move_table();

    s += &table_string(&bishop_table, "BISHOP_TABLE");

    //Write to text file
    let mut file = File::create("generated_files/perfect_hashing.rs").unwrap();
    file.write_all(s.as_bytes()).unwrap();
}

pub fn table_string(table: &Vec<HashMap<u64, u64>>, name: &str) -> String {
    let mut s = String::new();

    s += &format!("pub static {}: [Map<u64, u64>; 64] = [\n", name);

    for i in 0..64 {
        s += &format!("\tphf_map! {{\n");
        for (key, value) in table[i].iter() {
            s += &format!("\t\t0x{:016x}_u64 => 0x{:016x}_u64,\n", key, value);
        }
        s += &format!("}},\n");
    }

    s += &format!("];\n");

    return s;
}

pub fn gen_rook_move_table() -> Vec<HashMap<u64, u64>> {
    let mut ret = Vec::new();
    let rook_blocker_mask = ROOK_BLOCKER_MASK;

    for s in VALID_SQUARES {        
        let mask = rook_blocker_mask[s as usize];
        let bit_count = mask.count_ones();
        
        let mut map = HashMap::new();
        for index in 0..(1_u64 << bit_count) {
            let blocker = bitintr::Pdep::pdep(index, mask);
            
            let moves = gen_rook_moves(s, 0, blocker);
            
            map.insert(blocker as u64, moves);
        }

        ret.push(map);
    }

    ret
}

pub fn gen_bishop_move_table() -> Vec<HashMap<u64, u64>> {
    let mut ret = Vec::new();
    let bishop_blocker_mask = BISHOP_BLOCKER_MASK;

    for s in VALID_SQUARES {        
        let mask = bishop_blocker_mask[s as usize];
        let bit_count = mask.count_ones();
        
        let mut map = HashMap::new();
        for index in 0..(1_u64 << bit_count) {
            let blocker = bitintr::Pdep::pdep(index, mask);
            
            let moves = gen_bishop_moves(s, 0, blocker);
            
            map.insert(blocker as u64, moves);
        }

        ret.push(map);
    }

    ret
}