# Notes

## UI

- Rendering
- Move making
- Files and Ranks
- Flip for black view
- Undo move
- Multithreaded rendering

## Barschbot

- Endgame table with undo_move enumeration
- Opening book with lichess database

## Benchmarking

- `cargo rustc --bin benchmark --release -- -C target-cpu=native`

## History

- Initial: 151 s
- InBetweenTable: 137 s
- Blsr iterator: 150 s
- Check, Pin mask: 100 s
- Pext slider gen: 100 s
- Vertical translation: 100 s
- BitArray trait: 95 s
- ArrayVec: 86 s

### Algorithmic

- `blsr` for BitArray iterator
- `pext` for slider
- move list with `ArrayVec<(BitArray, Square)>`
  - iterator for usage
  - custom iterator for captures only
- dynamic attack board
- const expr `<const WHITE: bool>`

### Refactors

- `u8` vs  `i8`
- `BitArray` vs `u64`
- `Square` vs `i8`
