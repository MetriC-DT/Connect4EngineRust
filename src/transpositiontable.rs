use crate::{board::Board, moves::EMPTY_MOVE};

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
pub const MAX_TABLE_SIZE: usize = 8388593;

/// number of bits used to store the key (refer to explanation above why we don't use all 49 bits.)
const STORED_KEY_BITS: u64 = 32;

/// 32 bit mask for finding the key bits to store.
const STORED_KEY_BIT_MASK: u64 = (1 << STORED_KEY_BITS) - 1;

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
    flag: u8,
    depth: u8,
    mv: u8
}

pub type Flag = u8;
pub const FLAG_UPPER: Flag = 0;
pub const FLAG_LOWER: Flag = 1;
pub const FLAG_EXACT: Flag = 2;

impl Entry {
    pub fn new(board_key: u64, eval: i8, flag: Flag, depth: u8, mv: u8) -> Self {
        let stored_key = u32::try_from(board_key & STORED_KEY_BIT_MASK).unwrap();
        Self { stored_key, eval, flag, depth, mv }
    }

    pub fn get_key(&self) -> u32 {
        self.stored_key
    }

    pub fn get_eval(&self) -> i8 {
        self.eval
    }

    pub fn get_flag(&self) -> Flag {
        self.flag
    }

    pub fn get_depth(&self) -> u8 {
        self.depth
    }

    pub fn get_mv(&self) -> u8 {
        self.mv
    }

    pub fn clear(&mut self) {
        self.stored_key = EMPTY_KEY;
    }
}

impl Default for Entry {
    fn default() -> Self {
        Entry::new(EMPTY_KEY as u64, i8::MIN, FLAG_UPPER, u8::MAX, EMPTY_MOVE)
    }
}

#[derive(Debug)]
pub struct TranspositionTable {
    /// each entry of the table consists of 2 entries, with 2 different replacement policies:
    /// table_entry.0 = entry that is always replaced by new entries.
    /// table_entry.1 = replacement only happens when new entry has depth < existing.
    ///
    /// since depth == moves_made, a smaller moves_made means we can scan less of the
    /// tree if we cache that result.
    table: Vec<(Entry, Entry)>
}

impl TranspositionTable {

    /// initializes the new Transposition Table
    pub fn new() -> Self {
        let entries = ( Entry::default(), Entry::default() );
        Self { table: vec![entries; MAX_TABLE_SIZE] }
    }

    /// inserts the board game state and evaluation into the transposition table.
    pub fn insert(&mut self, board: &Board, eval: i8, flag: Flag, depth: u8, mv: u8) {
        let key = board.get_unique_position_key();
        self.insert_with_key(key, eval, flag, depth, mv);
    }

    /// inserts the board game state and eval into transposition table using key.
    pub fn insert_with_key(&mut self, key: u64, eval: i8, flag: Flag, depth: u8, mv: u8) {
        let entry = Entry::new(key, eval, flag, depth, mv);
        let loc = TranspositionTable::location(key);
        self.table[loc].0 = entry.clone(); // always replace

        // replace 1 entry only if depth is lower.
        let orig_entry = &self.table[loc].1;
        if depth < orig_entry.get_depth() {
            self.table[loc].1 = entry;
        }
    }

    /// obtains the location of the key into the transposition table.
    pub fn location(key: u64) -> usize {
        let loc = key % u64::try_from(MAX_TABLE_SIZE).unwrap();
        return loc.try_into().unwrap()
    }

    /// Gets the entry using the given board to calculate the key.
    /// bool determines whether the key matches (whether entry is valid).
    pub fn get_entry(&self, board: &Board) -> Option<&Entry> {
        let key = board.get_unique_position_key();
        self.get_entry_with_key(key)
    }

    pub fn get_exact_entry(&self, board: &Board) -> Option<&Entry> {
        let key = board.get_unique_position_key();
        let new_key = (key & STORED_KEY_BIT_MASK) as u32;
        let loc = TranspositionTable::location(key);
        let entry = &self.table[loc];

        if entry.0.get_key() == new_key && entry.0.get_flag() == FLAG_EXACT {
            return Some(&entry.0);
        } else if entry.1.get_key() == new_key && entry.1.get_flag() == FLAG_EXACT {
            return Some(&entry.1);
        } else {
            return None;
        }
    }

    /// obtains the selected entry, given a key.
    pub fn get_entry_with_key(&self, key: u64) -> Option<&Entry> {
        let loc = TranspositionTable::location(key);
        let (entry0, entry1) = &self.table[loc];
        let new_key = (key & STORED_KEY_BIT_MASK) as u32;

        if entry0.get_key() == new_key {
            return Some(entry0);
        } else if entry1.get_key() == new_key {
            return Some(entry1);
        } else {
            return None
        }
    }

    pub fn clear(&mut self) {
        for entry in self.table.iter_mut() {
            entry.0.clear();
            entry.1.clear();
        }
    }
}
