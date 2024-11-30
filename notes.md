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

Position counting

- Initial: 151 s
- InBetweenTable: 137 s
- Blsr iterator: 150 s
- Check, Pin mask: 100 s
- Pext slider gen: 100 s
- Vertical translation: 100 s
- BitArray trait: 95 s
- ArrayVec: 86 s
- Move iterator 110 s

Move counting

- ArrayVec: 12.1 s
- Move Iterator 9.0 s
- AttackedBits: 8.5 s

Depth + 1

- Attacked bits: 331 s
- Knogge gen for AttackedBits: 299 s
- Knogge gen for MoveGen: 295 s
- PHF slider: 384 s

### Algorithmic

Todo

- dynamic attack board
- const expr `<const WHITE: bool>`

Done

- `blsr` for BitArray iterator
- `pext` for slider
- move list with `ArrayVec<(BitArray, Square)>`
  - iterator for usage
  - custom iterator for captures only
- AttackBits for kingmoves
- Perfect hashtable

### Refactors

Todo

- `Square` vs `i8`
- `u8` vs  `i8`

Done

- `BitArray` vs `u64`
