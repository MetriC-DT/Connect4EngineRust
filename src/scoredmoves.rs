// Connect4EngineRust, a strong solver for the connect-4 board game.
// Copyright (C) 2023 Derick Tseng
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::mem::MaybeUninit;
use crate::board::{Position, WIDTH};

/// a Moves-like iterator but with the moves stored from low scores to high.
/// Higher scores are returned from the iterator before lower ones.
#[derive(Clone)]
pub struct ScoredMoves<T> where T: std::marker::Copy {

    /// array of (Position, corresponding_col, position_score) for each possible move.
    move_scores: [MaybeUninit<(Position, u8, T)>; WIDTH as usize],

    /// number of elements for iterator to emit.
    size: usize,

    /// index of the iterator.
    ptr: usize
}

impl<T> ScoredMoves<T> where T: std::marker::Copy + std::cmp::PartialOrd {
    /// creates a new `ScoredMoves` iterator.
    pub fn new() -> Self {
        let move_scores = [MaybeUninit::uninit(); WIDTH as usize];
        let size = 0;
        let ptr = 0;
        Self { move_scores, size, ptr }
    }

    /// creates a new `ScoredMoves` iterator with an initial element in it.
    pub fn new_with(mv: Position, col: u8, score: T) -> Self {
        let mut move_scores = [MaybeUninit::uninit(); WIDTH as usize];
        let size = 1;
        let ptr = 0;
        move_scores[0].write((mv, col, score));
        Self { move_scores, size, ptr }
    }

    /// adds a new move, col, score triple in order.
    pub fn add(&mut self, mv: Position, col: u8, score: T) {
        let mut i = self.size;
        self.size += 1;

        // insert into the move_scores array such that moves with higher scores are stored at lower
        // indices of the array compared to lower scores. In the case of ties, we would insert
        // into the next higher index.
        //
        // We insert from the back of the array since on average, the order would be the same, or
        // similar enough to the order `add` was called, which would result in the least amount of
        // data being copied.
        unsafe {
            while i > 0 && self.move_scores[i-1].assume_init().2 < score {
                self.move_scores[i] = self.move_scores[i-1];
                i -= 1;
            }
        }

        self.move_scores[i].write((mv, col, score));
    }
}

impl<T> Iterator for ScoredMoves<T> where T: std::marker::Copy {
    type Item=(Position, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr < self.size {
            // all data below self.size should be initialized already.
            unsafe {
                let (mv, col, _) = self.move_scores[self.ptr].assume_init();
                self.ptr += 1;
                return Some((mv, col));
            }
        }

        None
    }
}
