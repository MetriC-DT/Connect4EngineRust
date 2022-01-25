use std::collections::HashMap;

use crate::{board::Board, moves::MoveEvalPair};

pub struct TranspositionTable {
    table: HashMap<u64, MoveEvalPair>
}

impl TranspositionTable {

    /// initializes the new Transposition Table
    ///
    /// TODO: Hash function implementation could probably be removed
    /// since I already have a 64 bit unique identifier for each board's
    /// position (look at `Board::get_unique_position_key()`)
    pub fn new() -> Self {
        Self { table: HashMap::new() }
    }

    /// inserts the board game state and evaluation
    /// into the transposition table.
    pub fn insert(&mut self, board: &Board, pair: MoveEvalPair) {
        let key = board.get_unique_position_key();
        self.table.insert(key, pair);
    }

    pub fn get(&self, board: &Board) -> Option<&MoveEvalPair> {
        let key = board.get_unique_position_key();
        self.table.get(&key)
    }
}
