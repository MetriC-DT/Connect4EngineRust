use crate::board::{Position, WIDTH};
use crate::moves::EMPTY_MOVE;

/// a Moves-like iterator but with the moves stored from low scores to high.
/// Higher scores are returned from the iterator before lower ones.
pub struct ScoredMoves {

    /// array of (Position, corresponding_col, position_score) for each possible move.
    move_scores: [(Position, u8, i8); WIDTH as usize],

    /// number of elements for iterator to emit.
    size: usize,

    ptr: usize
}

impl ScoredMoves {
    pub fn new() -> Self {
        let move_scores = [(0, EMPTY_MOVE, i8::MIN); WIDTH as usize];
        let size = 0;
        let ptr = 0;
        Self { move_scores, size, ptr }
    }

    pub fn new_with(mv: Position, col: u8, score: i8) -> Self {
        let move_scores = [(mv, col, score); WIDTH as usize];
        let size = 1;
        let ptr = 0;
        Self { move_scores, size, ptr }
    }

    pub fn add(&mut self, mv: Position, col: u8, score: i8) {
        self.move_scores[self.size] = (mv, col, score);
        self.size += 1;
    }
}

impl Iterator for ScoredMoves {
    type Item=(Position, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr < self.size {
            let (mv, col, _) = self.move_scores[self.ptr];
            self.ptr += 1;
            return Some((mv, col));
        }

        None
    }
}
