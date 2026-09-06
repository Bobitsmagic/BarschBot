use barschbot::tablebase;

fn main() {
    let list = tablebase::endgame_table::generate_piece_lists(2);

    for l in list {
        println!(
            "{}",
            l.iter()
                .map(|x| x.to_char().to_string())
                .collect::<Vec<_>>()
                .join("")
        )
    }
}
