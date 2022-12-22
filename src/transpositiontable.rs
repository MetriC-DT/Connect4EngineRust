use crate::{board::Board, moves::EMPTY_MOVE};

/// Number of elements in the table. Best to choose a prime.
const MAX_TABLE_SIZE: usize = 8388593;

/// bits to retain in key (must be greater than 49 bits.
/// to 42 (number of slots in board) + 7 (1 extra bit for number of columns)
const KEY_BITS: i64 = 50;

/// mask for the playable region
const KEY_BIT_MASK: i64 = (1 << KEY_BITS) - 1;

/// location of the lowest bits in evaluation
const EVAL_LOC: i64 = KEY_BITS;

/// Evals are 8 bits.
const EVAL_BIT_MASK: i64 = ((1 << 8) - 1) << EVAL_LOC;

/// location of the lowest flag bits
const FLAG_LOC: i64 = EVAL_LOC + 8;

/// Flag is 2 bits (one of enum lower, upper, exact)
const FLAG_BIT_MASK: u64 = ((1 << 2) - 1) << FLAG_LOC;

/// location of the move
const MOVE_LOC: i64 = FLAG_LOC + 2;

/// move is 3 bits.
const MOVE_BIT_MASK: u64 = ((1 << 3) - 1) << MOVE_LOC;

/// represents an entry of the transposition table.
///
/// Storage format is {move (4), flag (2), eval (8), key (50)}
/// move is one of 0-6 (minimum 3 bits)
/// flag is one of LOWER, EXACT, UPPER
/// eval is the evaluation (8 bits)
/// key is the key of the board.
#[derive(Debug, Clone)]
pub struct Entry {
    storage: i64
}

pub type Flag = i8;
pub const FLAG_EXACT: Flag = 0;
pub const FLAG_UPPER: Flag = 1;
pub const FLAG_LOWER: Flag = 2;

impl Entry {
    pub fn new(board_key: i64, evaluation: i8, flag: Flag, mv: u8) -> Self {
        let mut storage = board_key & KEY_BIT_MASK;
        storage |= ((evaluation as i64) << EVAL_LOC) & EVAL_BIT_MASK;
        storage |= (flag as i64) << FLAG_LOC;
        storage |= (mv as i64) << MOVE_LOC;
        Self { storage }
    }

    pub fn get_key(&self) -> i64 {
        self.storage & KEY_BIT_MASK
    }

    pub fn get_eval(&self) -> i8 {
        let eval = (self.storage & EVAL_BIT_MASK) >> EVAL_LOC;
        let byte = eval.to_le_bytes()[0];
        i8::from_le_bytes([byte])
    }

    pub fn get_flag(&self) -> Flag {
        let flag = (self.storage as u64 & FLAG_BIT_MASK) >> FLAG_LOC;
        flag as i8
    }

    pub fn get_move(&self) -> u8 {
        let mv = (self.storage as u64 & MOVE_BIT_MASK) >> MOVE_LOC;
        mv as u8
    }
}

impl Default for Entry {
    fn default() -> Self {
        Entry::new(-1, 0, FLAG_EXACT, EMPTY_MOVE)
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

    /// inserts the board game state and evaluation into the transposition table.
    pub fn insert(&mut self, board: &Board, eval: i8, flag: Flag, mv: u8) {
        let key = board.get_unique_position_key();
        self.insert_with_key(key, eval, flag, mv);
    }

    /// inserts the board game state and eval into transposition table using key.
    pub fn insert_with_key(&mut self, key: i64, eval: i8, flag: Flag, mv: u8) {
        let entry = Entry::new(key, eval, flag, mv);
        self.table[TranspositionTable::location(key)] = entry;
    }

    /// obtains the location of the key into the transposition table.
    pub fn location(key: i64) -> usize {
        let keybytes = key.to_le_bytes();
        usize::from_le_bytes(keybytes) % MAX_TABLE_SIZE
    }

    /// Gets the entry using the given board to calculate the key.
    pub fn get_entry(&self, board: &Board) -> Option<&Entry> {
        let key = board.get_unique_position_key();
        self.get_entry_with_key(key)
    }

    /// obtains the selected entry, given a key.
    pub fn get_entry_with_key(&self, key: i64) -> Option<&Entry> {
        let loc = TranspositionTable::location(key);
        let selected_entry = &self.table[loc];

        if selected_entry.get_key() == key { Some(selected_entry) }
        else { None }
    }
}
