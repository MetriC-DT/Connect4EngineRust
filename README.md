# Connect4EngineRust
A strong solver for Connect 4 built with rust. Inspiration from [here](https://github.com/PascalPons/connect4).

## Optimizations and Features
- [x] Principal Variation Search (with alpha-beta pruning)
- [x] Bitboard
- [x] Move ordering
    - [x]  center-first to edge columns
    - [x]  critical moves first
    - [x]  position heuristic
    - [ ] Refutation Move (with Transposition Table)
- [x] Transposition Table (Two-Tiered)
- [x] Iterative Deepening
- [ ] Saves Principal Variation
- [ ] NNUE
- [ ] Multithreaded search
- [ ] Openings database
