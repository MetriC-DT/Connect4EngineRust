use crate::board::{WIDTH, Position};
use anyhow::Result;

/// Helper to generate a database of random legal positions for use in training the NNUE and
/// perhaps in generating a good openings database.
pub struct Database {
    /// number of elements in the array before we need to write to file.
    batch_size: usize,

    batch: Vec<DBEntry>
}

impl Database {

    /// initializes a new database.
    pub fn new_with_batch_size(batch_size: usize) -> Self {
        let batch = Vec::with_capacity(batch_size);
        Self { batch_size, batch }
    }

    /// writes the number of entries to a file. Expects the filename to be a sqlite3 database
    pub fn write_entries(num_entries: usize, filename: &str) -> Result<()> {
        Ok(())
    }
}

/// default initializer for a database.
impl Default for Database {
    fn default() -> Self {
        Self::new_with_batch_size(128)
    }
}


/// Each DBEntry contains information about the position. The `current player` represents the
/// player who had just played the last move. `opponent player` is the player who is next to move
/// (but has not played their move yet).
struct DBEntry {
    move_history: String,

    /// number of moves played on the board.
    moves_played: u8,

    /// Bitboard of the player who just made a move (the current player).
    player_board: Position,

    /// Bitboard of the player who is next to move (the opponent player).
    opp_board: Position,

    /// evaluation score, if it exists.
    eval: Option<i8>
}
