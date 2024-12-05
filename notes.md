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

### Depth 4

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

NegaScout:

- Nodes: 626283
- Evals: 483698
- Time: 444

### Depth 6

NegaAlphaBeta:

- Nodes: 24224135
- Evals: 18568434
- Time: 18961

NegaScout:

- Nodes: 21397158
- Evals: 16604698
- Time: 16417

Move sorter with check:

NegaAlphaBeta:

- Nodes: 20611073
- Evals: 15987200
- Time: 16753

NegaScout:

- Nodes: 17484697
- Evals: 13632489
- Time: 14031

NegaScout + Iterative deepening + Transposition Table

- Time: 9140 ms
- Nodes: 11030822
- Evals: 8219554

NegaScout + Iterative deepening + Transposition Table + Quiet move table

- Time: 7404 ms
- Nodes: 8991583
- Evals: 6537089

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
