use std::fmt;

use crate::moves::Moves;

/// Height of the connect 4 board
pub const HEIGHT: u8 = 6;

/// Width of the connect 4 board.
pub const WIDTH: u8 = 7;

/// Total number of elements of the board.
pub const SIZE: u8 = WIDTH * HEIGHT;

/// down, up-left, left, down-left directions of bitboard
pub const DIRECTION: [usize; 4] = [1, 6, 7, 8];

/// bit representation of the playable board.
pub const PLAYABLE_REGION: i64 = 0b0111111011111101111110111111011111101111110111111;

/// mask for bottom row.
pub const BOTTOM_ROW_MASK: i64 = 0b0000001000000100000010000001000000100000010000001;

pub const COLUMN_MASK: i64 = (1 << HEIGHT) - 1;

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
/// the skip by 1 for each row is to make winner checking easier.
///
/// The `total_board` variable describes the OR of the two player's
/// bitboards.
///
/// The `board` variable describes the bitboard for player 1. In order
/// to obtain the bitboard for player 0, we can XOR it with `total_board`.
#[derive(Debug, Clone, Copy)]
pub struct Board {
    board: i64,
    total_board: i64,
    moves_made: u8
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
            moves_made: 0,
        }
    }

    /// initializes board with given position.
    ///
    /// The position string is a string that begins with all numbers [0-9]. The instance that a
    /// non-numerical character is encountered, this function will end and stop adding any more
    /// pieces to the board.
    pub fn new_position(position: &str) -> Self {
        let mut board = Board::new();
        for c in position.chars() {
            if let Some(col) = c.to_digit(10) {
                board.add(col as u8 - 1).unwrap();
            }
            else {
                break;
            }
        }
        board
    }

    /// Obtains the height of the specified column.
    ///
    /// 0 <= col < WIDTH
    pub fn get_height(&self, col: u8) -> u8 {
        let colmask: i64 = ((1 << HEIGHT) - 1) << (col * (HEIGHT + 1));
        (self.total_board & colmask).count_ones() as u8
    }

    /// obtains the value at the given row and col, if it exists.
    /// `row` counts from the bottom. Therefore, the bottommost row
    /// is equal to row `0` and the topmost is row `5`
    ///
    /// returns `true` if occupied, otherwise, `false`
    pub fn get(&self, row: u8, col: u8) -> Option<bool> {
        let mask = 1 << (row + col * (HEIGHT + 1));
        let piece = (self.board & mask) != 0;
        let within_total = (self.total_board & mask) != 0;

        if within_total {
            Some(piece)
        }
        else {
            None
        }
    }

    /// returns `true` if a new piece can be added into 
    /// the specified column.
    pub fn can_add(&self, col: u8) -> bool {
        !Board::col_is_occupied(self.total_board, col) && col < WIDTH
    }

    pub fn col_is_occupied(board: i64, col: u8) -> bool {
        let top_bit = 1 << ((HEIGHT - 1) + col * (HEIGHT + 1));
        (board & top_bit) != 0
    }

    /// adds a piece into the specified column. If operation cannot be done,
    /// then it throws an error.
    pub fn add(&mut self, col: u8) -> Result<(), &str> {
        if self.can_add(col) {
            self.add_unchecked(col);
            Ok(())
        } else {
            Err("Unable to add")
        }
    }

    /// performs the add operation assuming that the selected column
    /// can be added to.
    ///
    /// Undefined behavior if col cannot be added to.
    pub fn add_unchecked(&mut self, col: u8) {
        // updates the board
        self.set_next_available(col);

        // adds to history of moves
        self.moves_made += 1;
    }

    /// sets the next available bit at `col`
    fn set_next_available(&mut self, col: u8) {
        let shift = col * (HEIGHT + 1);

        // mask for the bottom of the column `col`. If we add
        // this to total_board, then we can get the location of the
        // next available slot in the column.
        let mask = 1 << shift;
        let col_mask = COLUMN_MASK << shift;
        let new_position = (self.total_board & col_mask) + mask;
        self.total_board ^= new_position;

        // (1 or 0) * new_position is faster than the if statement...
        self.board ^= -(self.get_current_player() as i64) & new_position;
    }

    /// returns true if the bitboard is a winner.
    ///
    /// We do not need an option for checking if this current player has lost
    /// because you cannot lose the game on the turn you played your move.
    fn is_win(bitboard: i64) -> bool {
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
    pub fn get_bitboard_str(bitboard: u64) -> String {
        let mut s = String::with_capacity((SIZE + HEIGHT) as usize);
        for i in 0..SIZE {
            let c = i % WIDTH;
            let r = HEIGHT - i / WIDTH - 1;
            let mask = 1 << (r + c * (HEIGHT + 1));

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

    /// determines whether the first player has won
    pub fn is_first_player_win(&self) -> bool {
        let p1board = self.board ^ self.total_board;
        Board::is_win(p1board)
    }

    pub fn is_second_player_win(&self) -> bool {
        let p2board = self.board;
        Board::is_win(p2board)
    }

    /// puts the valid moves into the given moves_vec
    pub fn get_valid_moves(&self) -> Moves {
        Moves::new(self.total_board)
    }

    /// checks whether the entire board is entirely filled.
    pub fn is_filled(&self) -> bool {
        self.moves_made == SIZE
    }

    /// obtains the number of moves made.
    pub fn moves_played(&self) -> u8 {
        self.moves_made
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
    pub fn get_unique_position_key(&self) -> i64 {
        // OLD WAY
        // let bounding_limits = self.total_board + BOTTOM_ROW_MASK;
        // bounding_limits ^ self.board
        
        // the old way had me adding BOTTOM_ROW_MASK in the calculation for
        // unique position key. This is just a wasted instruction and can be
        // removed.
        self.total_board + self.board
    }

    pub fn get_num_moves_played(&self) -> u8 {
        self.moves_made
    }

    /// returns true if game is over, false otherwise.
    pub fn is_game_over(&self) -> bool {
        self.is_first_player_win() ||
            self.is_second_player_win() ||
            self.is_filled()
    }

    /// obtains the current player ID (either 0 or 1).
    pub fn get_current_player(&self) -> u8 {
        self.moves_made & 1
    }
}
