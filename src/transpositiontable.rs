use crate::board::Board;

/// Number of elements in the table. Best to choose a prime.
const MAX_TABLE_SIZE: usize = (1 << 23) + 9;

/// represents an entry of the transposition table.
#[derive(Debug, Clone)]
struct Entry {
    board_key: i64,
    evaluation: i8
}

impl Entry {
    pub fn new(board_key: i64, evaluation: i8) -> Self {
        Self { board_key, evaluation }
    }
}

impl Default for Entry {
    fn default() -> Self {
        // chose to use -1 (0b11111...) because no key will have that value.
        Entry::new(-1, 0)
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

        if selected_entry.board_key == key {
            Some(selected_entry.evaluation)
        }
        else {
            None
        }
    }
}
