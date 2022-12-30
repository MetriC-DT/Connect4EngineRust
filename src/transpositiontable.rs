use crate::board::Board;

/// Using the Chinese remainder theorem, using our key (which could be encoded in 49 bits), the
/// two co-prime divisors are 2^(STORED_KEY_BITS) and MAX_TABLE_SIZE. Hence,
/// key === a mod (2^STORED_KEY_BITS) and
/// key === b mod (MAX_TABLE_SIZE)
/// Since `2^STORED_KEY_BITS` and `MAX_TABLE_SIZE` are chosen to be pairwise co-prime, if `key` is
/// a natural number < (2^STORED_KEY_BITS * MAX_TABLE_SIZE), then it will have a unique `c` where:
/// key === c mod (2^STORED_KEY_BITS * MAX_TABLE_SIZE)
/// Thus, the key can be uniquely determined by the pair (a, b) if the key is less than
/// (2^STORED_KEY_BITS * MAX_TABLE_SIZE). Since b is used as the location of the entry in the
/// table, all we need to do is store a (which requires `STORED_KEY_BITS` bits).

/// Number of elements in the table. Best to choose a prime, and must be odd.
/// With the Chinese remainder theorem, the size must be greater than 2^17, since the
/// STORED_KEY_BITS is 32 bits and we need to uniquely encode 2^49 numbers (49-32=17), where 2^49 is
/// number of different keys to encode.
const MAX_TABLE_SIZE: usize = 8388593;

/// number of bits used to store the key (refer to explanation above why we don't use all 49 bits.)
const STORED_KEY_BITS: u64 = 32;

/// 32 bit mask for finding the key bits to store.
const STORED_KEY_BIT_MASK: u64 = (1 << STORED_KEY_BITS) - 1;

/// flag bits (lowest 2 bits of the metadata)
const FLAG_BIT_MASK: u8 = 0b11;

/// empty key.
const EMPTY_KEY: u32 = u32::MAX;

/// represents an entry of the transposition table.
///
/// stored_key: lower 32 bit of 49-bit board key.
/// eval: evaluation of the position.
/// metadata: { MOVE (u3), FLAG (u2) }
#[derive(Debug, Clone)]
pub struct Entry {
    stored_key: u32,
    eval: i8,
    metadata: u8
}

pub type Flag = u8;
pub const FLAG_UPPER: Flag = 0;
pub const FLAG_LOWER: Flag = 1;

impl Entry {
    pub fn new(board_key: u64, eval: i8, flag: Flag) -> Self {
        let stored_key = u32::try_from(board_key & STORED_KEY_BIT_MASK).unwrap();
        let metadata = flag;
        Self { stored_key, eval, metadata }
    }

    pub fn get_key(&self) -> u32 {
        self.stored_key
    }

    pub fn get_eval(&self) -> i8 {
        self.eval
    }

    pub fn get_flag(&self) -> Flag {
        self.metadata & FLAG_BIT_MASK
    }

    pub fn wipe(&mut self) {
        self.stored_key = EMPTY_KEY;
    }
}

impl Default for Entry {
    fn default() -> Self {
        Entry::new(EMPTY_KEY as u64, i8::MIN, FLAG_UPPER)
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
    pub fn insert(&mut self, board: &Board, eval: i8, flag: Flag) {
        let key = board.get_unique_position_key();
        self.insert_with_key(key, eval, flag);
    }

    /// inserts the board game state and eval into transposition table using key.
    pub fn insert_with_key(&mut self, key: u64, eval: i8, flag: Flag) {
        let entry = Entry::new(key, eval, flag);
        let loc = TranspositionTable::location(key);
        self.table[loc] = entry;
    }

    /// obtains the location of the key into the transposition table.
    pub fn location(key: u64) -> usize {
        let loc = key % u64::try_from(MAX_TABLE_SIZE).unwrap();
        return loc.try_into().unwrap()
    }

    /// Gets the entry using the given board to calculate the key.
    /// bool determines whether the key matches (whether entry is valid).
    pub fn get_entry(&self, board: &Board) -> (&Entry, bool) {
        let key = board.get_unique_position_key();
        self.get_entry_with_key(key)
    }

    /// obtains the selected entry, given a key.
    pub fn get_entry_with_key(&self, key: u64) -> (&Entry, bool) {
        let loc = TranspositionTable::location(key);
        let entry = &self.table[loc];
        let valid = (key & STORED_KEY_BIT_MASK) as u32 == entry.get_key();
        return (entry, valid)
    }
}
