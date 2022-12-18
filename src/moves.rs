use crate::board::{WIDTH, Board};

const DEFAULT_ORDER: [u8; WIDTH as usize] = [3, 2, 4, 1, 5, 0, 6];

/// representation for the allowed moves on a player's turn.
///
/// REQUIRES that the given game has not been completed yet.
/// If game is already completed, will result in undefined behavior.
///
/// AGAIN, MAKE SURE GAME HAS NOT BEEN COMPLETED YET!!!
#[derive(Clone, Copy)]
pub struct Moves {
    total_board: i64,
    pointer: usize,
}

impl Moves {
    pub fn new(total_boards: i64) -> Self {
        Self { total_board: total_boards, pointer: 0 }
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

#[derive(Clone, Copy, Debug)]
pub struct MoveEvalPair(u8, i8);

impl MoveEvalPair {
    pub fn new(mv: u8, score: i8) -> Self {
        Self(mv, score)
    }
    pub fn set_eval(&mut self, eval: i8) {
        self.1 = eval;
    }

    pub fn set_move(&mut self, mv: u8) {
        self.0 = mv;
    }

    pub fn get_eval(&self) -> i8 {
        self.1
    }

    pub fn get_move(&self) -> u8 {
        self.0
    }
}
