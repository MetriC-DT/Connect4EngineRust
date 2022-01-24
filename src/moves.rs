use crate::board::{WIDTH, Board};

const ORDER: [usize; WIDTH] = [3, 2, 4, 1, 5, 0, 6];

/// representation for the allowed moves on a player's turn.
#[derive(Clone, Copy)]
pub struct Moves {
    board: u64,
    pointer: usize,
}

impl Moves {
    pub fn new(bits: u64) -> Self {
        Self { board: bits, pointer: 0 }
    }
}

/// iterator over the possible moves that can be played.
impl Iterator for Moves {
    type Item=usize;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.pointer;
        for &col in &ORDER[i..] {
            self.pointer += 1;
            if !Board::col_is_occupied(self.board, col) {
                return Some(col);
            }
        }

        None
    }
}
