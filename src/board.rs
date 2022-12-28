use crate::moves::Moves;
use std::fmt;
use anyhow::{Result, bail};

pub type Position = u64;

/// Height of the connect 4 board
pub const HEIGHT: u8 = 6;

/// Width of the connect 4 board.
pub const WIDTH: u8 = 7;

/// Total number of elements of the board.
pub const SIZE: u8 = WIDTH * HEIGHT;

/// bit representation of the playable board.
pub const PLAYABLE_REGION: Position = 0b0111111_0111111_0111111_0111111_0111111_0111111_0111111;

/// mask for bottom row.
pub const BOTTOM_ROW_MASK: Position = 0b0000001_0000001_0000001_0000001_0000001_0000001_0000001;

/// mask for top row.
pub const TOP_ROW_MASK: Position = 0b0100000_0100000_0100000_0100000_0100000_0100000_0100000;

/// mask for a column (0b111111)
pub const COLUMN_MASK: Position = (1 << HEIGHT) - 1;

/// number of items in every column, including extra top bit.
pub const COUNTS_PER_COL: u8 = 7;

/// down, up-left, left, down-left directions of bitboard
pub const DIRECTION: [u8; 4] = [1, COUNTS_PER_COL - 1, COUNTS_PER_COL, COUNTS_PER_COL + 1];

/// Bitboard implementation of the Connect 4 Board.
/// 
/// The Board is represented as a 64 bit integer, with bits
/// flipped to `1` for the slots that are occupied, and `0`
/// for bits that are not for each color. Therefore, each
/// player gets their own bitboard.
///
/// Each bitboard is represented as such:
/// 5 12 19 26 33 40 47
/// 4 11 18 25 32 39 46
/// 3 10 17 24 31 38 45
/// 2 9  16 23 30 37 44
/// 1 8  15 22 29 36 43
/// 0 7  14 21 28 35 42
///
/// the skip bits is to make winner checking easier
/// (make sure we cannot create a win by having bits 4, 5, 6, 7, for example)
///
/// The `total_board` variable describes the OR of the two player's
/// bitboards.
///
/// The `board` variable describes the bitboard for the current player.
#[derive(Debug, Clone, Copy)]
pub struct Board {
    board: Position,
    total_board: Position,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity((SIZE + HEIGHT) as usize);
        for i in 0..SIZE {
            let c = i % WIDTH;
            let r = HEIGHT - i / WIDTH - 1;

            match self.get(r, c) {
                Some(false) => s.push('O'),
                Some(true) => s.push('X'),
                None => s.push('_'),
            };

            if (i + 1) % WIDTH == 0 {
                s.push('\n');
            };
        }

        write!(f, "{}", s)
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

impl Board {
    /// Creates a new board.
    pub fn new() -> Self {
        Self {
            board: 0,
            total_board: 0,
        }
    }

    /// initializes board with given position.
    ///
    /// The position string is a string that begins with col numbers [1-7]. The instance that a
    /// non-numerical character is encountered, this function will end and stop adding any more
    /// pieces to the board. (example position string in `test_inputs/`)
    pub fn new_position(position: &str) -> Result<Self> {
        let mut board = Board::new();
        for (i, c) in position.chars().enumerate() {
            let mv = c.to_digit(10);
            if let None = mv {
                bail!("Invalid character in position {}", i);
            }

            let mv = mv.unwrap().checked_sub(1);
            if let None = mv {
                bail!("Invalid character in position {}", i);
            }

            let col: Result<u8, _> = mv.unwrap().try_into();
            if let Err(_) = col {
                bail!("Invalid character in position {}", i);
            }

            if let Err(_) = board.add(col.unwrap()) {
                bail!("Invalid character in position {}", i);
            }
        }

        Ok(board)
    }

    /// obtains the value at the given row and col, if it exists.
    /// `row` counts from the bottom. Therefore, the bottommost row
    /// is equal to row `0` and the topmost is row `5`
    ///
    /// returns `true` if occupied, otherwise, `false`
    pub fn get(&self, row: u8, col: u8) -> Option<bool> {
        let board = if self.moves_played() % 2 == 0 {
            self.board
        } else {
            self.board  ^ self.total_board
        };

        let mask = 1 << (row + col * COUNTS_PER_COL);
        let piece = (board & mask) != 0;
        let within_total = (self.total_board & mask) != 0;

        if within_total {
            Some(piece)
        }
        else {
            None
        }
    }

    /// used only for testing purposes. Should not use.
    pub fn get_height(&self, col: u8) -> u8 {
        let col_mask = COLUMN_MASK << (col * COUNTS_PER_COL);
        let board_column = col_mask & self.total_board;
        board_column.count_ones() as u8
    }

    /// returns `true` if a new piece can be added into 
    /// the specified column.
    fn can_add(&self, col: u8) -> bool {
        col < WIDTH && !Board::col_is_occupied(self.total_board, col)
    }

    /// returns true if the entire column is occupied.
    fn col_is_occupied(board: Position, col: u8) -> bool {
        let col_mask = COLUMN_MASK << (col * COUNTS_PER_COL);
        let top_bit = TOP_ROW_MASK & col_mask;
        (board & top_bit) != 0
    }

    /// adds a piece into the specified column. If operation cannot be done,
    /// then it throws an error.
    pub fn add(&mut self, col: u8) -> Result<()> {
        if self.can_add(col) {
            let possible = self.possible_moves();
            let pos = Board::col_to_pos(possible, col);
            self.play(pos);
            Ok(())
        } else {
            bail!("Unable to add to column")
        }
    }

    /// converts the column [0, 6] to the bit position to play.
    pub fn col_to_pos(possible: Position, col: u8) -> Position {
        let col_mask = COLUMN_MASK << (col * COUNTS_PER_COL);
        possible & col_mask
    }

    /// converts the bit position to play into a column.
    pub fn pos_to_col(p: Position) -> u8 {
        p.trailing_zeros() as u8 / COUNTS_PER_COL
    }

    /// whether a given position move could be played.
    pub fn valid_play_pos(pos: Position) -> bool {
        pos != 0
    }

    /// obtains the position of possible moves. E.g.
    ///
    /// Total board:
    /// _ _ _ _ _ _ _
    /// _ _ _ _ _ _ _
    /// _ _ _ _ _ _ _
    /// _ _ O _ _ _ _
    /// _ O X O _ _ _
    /// X O X X _ _ _
    ///
    /// Possible moves are:
    /// 0 0 0 0 0 0 0
    /// 0 0 0 0 0 0 0
    /// 0 0 1 0 0 0 0
    /// 0 1 0 1 0 0 0
    /// 1 0 0 0 0 0 0
    /// 0 0 0 0 1 1 1
    pub fn possible_moves(&self) -> Position {
        (self.total_board + BOTTOM_ROW_MASK) & PLAYABLE_REGION
    }

    /// performs the add operation assuming that the selected position can be played.
    /// Undefined behavior if position is not valid.
    pub fn play(&mut self, pos: Position) {
        // updates the board to the current player.
        self.board ^= self.total_board;

        // updates the board
        self.total_board |= pos;
        self.board |= pos;
    }

    pub fn revert(&mut self, pos: Position) {
        // reverts the added position.
        self.total_board ^= pos;
        self.board ^= pos;

        self.board ^= self.total_board;
    }

    /// returns true if the bitboard is a winner.
    ///
    /// We do not need an option for checking if this current player has lost
    /// because you cannot lose the game on the turn you played your move.
    pub fn is_win(bitboard: Position) -> bool {
        for dir in DIRECTION {
            // checks two at a time for better efficiency.
            let bb = bitboard & (bitboard >> dir);
            if (bb & (bb >> (2 * dir))) != 0 {
                return true;
            }
        }

        false
    }

    /// obtains the string representation of a bitboard.
    pub fn get_bitboard_str(bitboard: Position) -> String {
        let mut s = String::with_capacity((SIZE + HEIGHT) as usize);
        for i in 0..SIZE {
            let c = i % WIDTH;
            let r = HEIGHT - i / WIDTH - 1;
            let mask = 1 << (r + c * COUNTS_PER_COL);

            if bitboard & mask != 0 {
                s.push('1');
            }
            else {
                s.push('0');
            }

            if (i + 1) % WIDTH == 0 {
                s.push('\n');
            };
        }
        s
    }

    /// returns the position from the current player's perspective.
    pub fn get_curr_player_pos(&self) -> Position {
        self.total_board ^ self.board
    }

    /// Returns a new position with `mv` played on `pos`.
    /// Assumes that mv can be played, and pos is valid. Undefined behavior if it is not.
    pub fn test_pos(pos: Position, mv: Position) -> Position {
        pos | mv
    }

    /// puts the valid moves into the given moves_vec
    pub fn get_valid_moves(&self) -> Moves {
        Moves::new(self.possible_moves())
    }

    /// checks whether the entire board is entirely filled.
    pub fn is_filled(&self) -> bool {
        self.total_board == PLAYABLE_REGION
    }

    pub fn has_winner(&self) -> bool {
        Board::is_win(self.board)
    }

    /// obtains the number of moves made.
    /// Should not continue to call in heavy calculations. Instead, it is recommended to add and
    /// subtract from a local variable as necessary whenever a move gets played.
    pub fn moves_played(&self) -> u32 {
        self.total_board.count_ones()
    }

    /// obtains the unique position key. This is calculated by
    /// obtaining the top bound of the total board for each column
    /// then shifting it upwards by 1, then xor with the player board.
    ///
    /// e.g. if
    /// player board:
    /// 0 0 0 0 0 0 0
    /// 0 1 0 0 1 0 0
    /// 1 0 0 0 0 0 0
    /// 0 1 1 0 1 0 0
    /// 0 1 1 0 1 0 0
    /// 1 0 0 0 0 0 0
    ///
    /// total board:
    /// 0 0 0 0 1 0 0
    /// 1 1 0 0 1 0 0
    /// 1 1 0 0 1 0 0
    /// 1 1 1 0 1 0 0
    /// 1 1 1 0 1 0 0
    /// 1 1 1 1 1 0 0
    ///
    /// top bound of total board:
    /// 0 0 0 0 1 0 0
    /// 1 1 0 0 0 0 0
    /// 0 0 0 0 0 0 0
    /// 0 0 1 0 0 0 0
    /// 0 0 0 0 0 0 0
    /// 0 0 0 1 0 0 0
    ///
    /// We shift the top bound up by 1 to get the bounding limits
    /// of the playable board. This works because a slot of `0` below the
    /// bounding limits implies that the slot is occupied by the first player,
    /// while zeroes above mean empty.
    pub fn get_unique_position_key(&self) -> u64 {
        // OLD WAY
        // let bounding_limits = self.total_board + BOTTOM_ROW_MASK;
        // bounding_limits ^ self.board
        
        // the old way had me adding BOTTOM_ROW_MASK in the calculation for
        // unique position key. This is just a wasted instruction and can be
        // removed.
        self.total_board + self.board
    }

    pub fn is_first_player_win(&self) -> bool {
        return self.has_winner() && (self.moves_played() % 2 == 1)
    }

    pub fn is_second_player_win(&self) -> bool {
        return self.has_winner() && (self.moves_played() % 2 == 0)
    }
}
