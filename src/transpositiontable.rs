use crate::board::Board;

/// Number of elements in the table. Best to choose a prime.
const MAX_TABLE_SIZE: usize = 8388593;

/// bits in playable region (refer to board.rs)
const PLAYABLE_BITS: i64 = 56;

/// mask for the playable region
const KEY_BIT_MASK: i64 = (1 << PLAYABLE_BITS) - 1;

/// whatever isn't the key mask will be the evaluation.
const EVAL_BIT_MASK: i64 = !KEY_BIT_MASK;

/// represents an entry of the transposition table.
#[derive(Debug, Clone)]
struct Entry {
    storage: i64
}

impl Entry {
    pub fn new(board_key: i64, evaluation: i8) -> Self {
        // we don't need to mask anything since board is guaranteed to not
        // have anything in the bits above the 53.
        let mut storage = board_key;
        storage |= (evaluation as i64) << PLAYABLE_BITS;
        Self { storage }
    }

    pub fn get_key(&self) -> i64 {
        self.storage & KEY_BIT_MASK
    }

    pub fn get_eval(&self) -> i8 {
        let eval = (self.storage & EVAL_BIT_MASK) >> PLAYABLE_BITS;
        eval.try_into().unwrap()
    }
}

impl Default for Entry {
    fn default() -> Self {
        Entry::new(0, 0)
    }
}

#[derive(Debug)]
pub struct TranspositionTable {
    table: Vec<Entry>
}

impl TranspositionTable {

    /// initializes the new Transposition Table
    pub fn new() -> Self {
        Self { table: vec![Entry::default(); MAX_TABLE_SIZE] }
    }

    /// inserts the board game state and evaluation
    /// into the transposition table.
    pub fn insert(&mut self, board: &Board, eval: i8) {
        let key = board.get_unique_position_key();
        let entry = Entry::new(key, eval);
        self.table[TranspositionTable::location(key)] = entry;
    }

    /// obtains the location of the key into the transposition table.
    pub fn location(key: i64) -> usize {
        let keybytes = key.to_le_bytes();
        usize::from_le_bytes(keybytes) % MAX_TABLE_SIZE
    }

    pub fn get(&self, board: &Board) -> Option<i8> {
        let key = board.get_unique_position_key();
        let loc = TranspositionTable::location(key);
        let selected_entry = &self.table[loc];

        if selected_entry.get_key() == key {
            Some(selected_entry.get_eval())
        }
        else {
            None
        }
    }
}
