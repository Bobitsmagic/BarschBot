use barschbot::board::square::EN_PASSANT_SQUARES;
use rand::{rngs::StdRng, RngCore, SeedableRng};

pub fn main() {
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(0);

    let en_passant = gen_en_passant(&mut rng);

    print_array("EN_PASSANT", &en_passant);
}

fn print_array(name: &str, array: &[u64; 64]) {
    println!("const {}: [u64; {}] = [", name, array.len());
    for y in 0..8 {
        print!("\t");
        for x in 0..8 {
            print!("0x{:016x}, ", array[x + y * 8]);
        }
        println!();
    }
    println!("];");
}

fn gen_en_passant(rng: &mut StdRng) -> [u64; 64] {
    let mut en_passant = [0_u64; 64];

    for s in EN_PASSANT_SQUARES {
        let index = s as usize;
        en_passant[index] = rng.next_u64();
    }

    return en_passant;
}
