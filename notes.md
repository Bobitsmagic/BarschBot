# Notes

## UI

Todo

- Flip for black view
- Undo move
- Arrow drawing
- Evalbar

Done

- Rendering
- Move making
- Files and Ranks
- Multithreaded rendering

## Barschbot

- Endgame table with undo_move enumeration
- Opening book with lichess database

## Benchmarking search

NegaMax:

- Nodes: 113068461
- Evals: 109625445
- Time: 95521

NegaAlphaBeta:

- Nodes: 2882936
- Evals: 2566585
- Time: 2216

NegaAlphaBeta + move Sorter:

- Nodes: 577339
- Evals: 434558
- Time: 407

## Benchmarking Move gen

- `cargo rustc --bin benchmark --release -- -C target-cpu=native`

### History

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
- No square: 287 s

### Algorithmic

Todo

- const expr `<const WHITE: bool>` //Not doable because there are no const expr?

Done

- dynamic attack board  (Big fail)
- `blsr` for BitArray iterator
- `pext` for slider
- move list with `ArrayVec<(BitArray, Square)>`
  - iterator for usage
  - custom iterator for captures only
- AttackBits for kingmoves
- Perfect hashtable

### Refactors

Todo

Done

- `u8` vs  `i8`
- `Square` vs `i8`
- `BitArray` vs `u64`
