use std::fmt;

use crate::moves::Moves;

/// Height of the connect 4 board
pub const HEIGHT: usize = 6;

/// Width of the connect 4 board.
pub const WIDTH: usize = 7;

/// Total number of elements of the board.
pub const SIZE: usize = WIDTH * HEIGHT;

/// down, up-left, left, down-left directions of bitboard
pub const DIRECTION: [usize; 4] = [1, 6, 7, 8];

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
#[derive(Debug)]
pub struct Board {
    board: u64,
    total_board: u64,
    history: Vec<usize>,
    nextplayer: bool,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity(SIZE + HEIGHT);
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

impl Board {
    pub fn new() -> Self {
        Self {
            board: 0,
            total_board: 0,
            history: Vec::with_capacity(SIZE as usize),
            nextplayer: false,
        }
    }

    /// Obtains the height of the specified column.
    ///
    /// 0 <= col < WIDTH
    pub fn get_height(&self, col: usize) -> usize {
        let colmask: u64 = ((1 << HEIGHT) - 1) << (col * (HEIGHT + 1)) as u64;
        (self.total_board & colmask).count_ones().try_into().unwrap()
    }

    /// obtains the value at the given row and col, if it exists.
    /// `row` counts from the bottom. Therefore, the bottommost row
    /// is equal to row `0` and the topmost is row `5`
    ///
    /// returns `true` if occupied, otherwise, `false`
    pub fn get(&self, row: usize, col: usize) -> Option<bool> {
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
    fn can_add(&self, col: usize) -> bool {
        let top_bit = 1 << ((HEIGHT - 1) + col * (HEIGHT + 1));
        (self.total_board & top_bit) == 0
    }

    /// adds a piece into the specified column. If operation cannot be done,
    /// then it throws an error.
    ///
    /// Also returns the winner of the board if possible.
    pub fn add(&mut self, col: usize) -> Result<(), &str> {
        if self.can_add(col) {
            let row = self.get_height(col);

            // updates the player's board
            self.flip(row, col);

            // adds to history of moves
            self.history.push(col);

            // switch to next player to play
            self.nextplayer = !self.nextplayer;
            Ok(())
        } else {
            Err("Unable to add")
        }
    }

    /// undoes the last move, if possible.
    pub fn undo(&mut self) {
        // pops latest entry from history
        if let Some(col) = self.history.pop() {
            let row = self.get_height(col) - 1;

            // sets the player to the previous player
            self.nextplayer = !self.nextplayer;

            // deletes the previous player's move
            self.flip(row, col);
        }
    }

    /// flips the bit set at `row` and `col`
    fn flip(&mut self, row: usize, col: usize) {
        let mask = 1 << (row + col * (HEIGHT + 1));
        let boardmask = (self.nextplayer as u64) << (row + col * (HEIGHT + 1));
        self.board ^= boardmask;
        self.total_board ^= mask;
    }

    /// returns true if the bitboard is a winner.
    ///
    /// We do not need an option for checking if this current player has lost
    /// because you cannot lose the game on the turn you played your move.
    pub fn is_win(bitboard: u64) -> bool {
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
        let mut s = String::with_capacity((SIZE + HEIGHT).into());
        for i in 0..SIZE {
            let c = i % WIDTH;
            let r = HEIGHT - i / WIDTH - 1;
            let mask = 1 << (r + c * (HEIGHT + 1));

            if bitboard & mask != 0 {
                s.push('1');
            } else {
                s.push('0');
            }
            if (i + 1) % WIDTH == 0 {
                s.push('\n');
            };
        }
        s
    }

    /// returns [player0 bitboard, player1 bitboard]
    pub fn get_bitboards(&self) -> [u64; 2] {
        [self.board ^ self.total_board, self.board]
    }

    /// puts the valid moves into the given moves_vec
    pub fn get_valid_moves(&self) -> Moves {
        let mut moves = Moves::new();

        // manually hardcode in the wanted ordering.
        for col in [0, 6, 1, 5, 2, 4, 3] {
            if self.can_add(col) {
                moves.add_move(col);
            }
        }

        moves
    }

    /// checks whether the entire board is entirely filled.
    pub fn is_filled(&self) -> bool {
        self.total_board.count_ones() as usize == SIZE
    }

    /// obtains the next player (player to make the move next).
    pub fn get_next_player(&self) -> bool {
        self.nextplayer
    }
}
