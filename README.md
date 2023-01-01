# Connect4EngineRust
A strong solver for Connect 4 built with rust. Inspiration from Pascal Pons' [Connect 4 Solver](https://github.com/PascalPons/connect4).

## Optimizations and Features
- [x] Principal Variation Search (with alpha-beta pruning)
- [x] Bitboard
- [x] Move ordering
    - [x]  center-first to edge columns
    - [x]  critical moves first
    - [x]  position heuristic
    - [x] Refutation Move (with Transposition Table)
- [x] Transposition Table (Two-Tiered)
- [ ] Gradual Widening Aspiration Window
- [ ] Saves Principal Variation
- [ ] NNUE
- [ ] Multithreaded search
- [ ] Openings database
