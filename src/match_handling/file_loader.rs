use std::{fs::File, io::Read};

use crate::game::game_state::GameState;

pub fn load_test_fens() -> Vec<GameState> {
    let mut fens = Vec::new();
    
    //read all lines from the file
    let mut file = File::open("data/Fens.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    //split the file into lines
    let lines = contents.split("\n");

    //parse each line into a game state
    for line in lines {
        fens.push(GameState::from_fen(line));
    }   

    fens
}