use std::fmt::Display;

use crate::board::{WIDTH, Board, Position};

pub const EMPTY_MOVE: u8 = u8::MAX;
pub const DEFAULT_ORDER: [u8; WIDTH as usize] = [3, 2, 4, 1, 5, 0, 6];

/// representation for the allowed moves on a player's turn.
///
/// REQUIRES that the given game has not been completed yet.
/// If game is already completed, will result in undefined behavior.
///
/// AGAIN, MAKE SURE GAME HAS NOT BEEN COMPLETED YET!!!
#[derive(Clone, Copy)]
pub struct Moves {
    possible: u64,
    pointer: usize,
}

impl Moves {
    pub fn new(possible: u64) -> Self {
        Self { possible, pointer: 0 }
    }
}

impl Display for Moves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let k = self.clone().collect::<Vec<Position>>();
        write!(f, "{:?}", k)
    }
}

/// iterator over the possible moves that can be played.
impl Iterator for Moves {
    type Item=Position;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.pointer;
        for &col in &DEFAULT_ORDER[i..] {
            self.pointer += 1;
            let play_pos = Board::col_to_pos(self.possible, col);
            if Board::valid_play_pos(play_pos) {
                return Some(play_pos)
            }
        }

        None
    }
}
