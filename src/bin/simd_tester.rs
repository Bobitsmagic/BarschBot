use barschbot::{game::game_state::GameState, moves::check_pin_mask};
use fearless_simd::{u64x2, u8x16, Level, Simd, SimdBase, SimdInto};
use fearless_simd_macros::simd;

fn broadcast<S: Simd>(simd: S, value: u64) -> u8x16<S> {
    let bytes: [u8; 16] = unsafe { std::mem::transmute([value, value]) };

    u8x16::from_slice(simd, bytes.as_slice())
}

fn main() {
    let mut gs = GameState::from_fen("rnbqkbnr/ppp1pppp/3p4/8/Q7/2P5/PP1PPPPP/RNB1KBNR b KQkq -");

    // check_pin_mask::CheckPinMask::pins_on(gs, board)
}
