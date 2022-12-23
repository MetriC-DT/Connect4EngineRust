use std::fmt::Display;

use crate::board::{WIDTH, Board};

pub const EMPTY_MOVE: u8 = u8::MAX;
const DEFAULT_ORDER: [u8; WIDTH as usize] = [3, 2, 4, 1, 5, 0, 6];

/// representation for the allowed moves on a player's turn.
///
/// REQUIRES that the given game has not been completed yet.
/// If game is already completed, will result in undefined behavior.
///
/// AGAIN, MAKE SURE GAME HAS NOT BEEN COMPLETED YET!!!
#[derive(Clone, Copy)]
pub struct Moves {
    total_board: u64,
    pointer: usize,
}

impl Moves {
    pub fn new(total_board: u64) -> Self {
        Self { total_board, pointer: 0 }
    }
}

impl Display for Moves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let k = DEFAULT_ORDER.iter()
            .filter(|x| !Board::col_is_occupied(self.total_board, **x))
            .collect::<Vec<_>>();
        write!(f, "{:?}", k)
    }
}

/// iterator over the possible moves that can be played.
impl Iterator for Moves {
    type Item=u8;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.pointer;
        for &col in &DEFAULT_ORDER[i..] {
            self.pointer += 1;
            if !Board::col_is_occupied(self.total_board, col) {
                return Some(col);
            }
        }

        None
    }
}
