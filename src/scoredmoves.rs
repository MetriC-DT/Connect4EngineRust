use crate::board::{Position, WIDTH};
use crate::moves::EMPTY_MOVE;

/// a Moves-like iterator but with the moves stored from low scores to high.
/// Higher scores are returned from the iterator before lower ones.
#[derive(Clone)]
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
        let mut i = self.size;
        self.size += 1;

        // insert into the move_scores array such that moves with higher scores are stored at lower
        // indices of the array compared to lower scores. In the case of ties, we would insert
        // into the next higher index.
        //
        // We insert from the back of the array since on average, the order would be the same, or
        // similar enough to the order `add` was called, which would result in the least amount of
        // data being copied.
        while i > 0 && self.move_scores[i-1].2 < score {
            self.move_scores[i] = self.move_scores[i-1];
            i -= 1;
        }
        self.move_scores[i] = (mv, col, score);
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
