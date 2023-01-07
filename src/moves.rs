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
    possible: Position,
    pointer: usize,
}

impl Moves {
    pub fn new(possible: Position) -> Self {
        Self { possible, pointer: 0 }
    }
}

/// iterator over the possible moves that can be played.
impl Iterator for Moves {
    type Item=(Position, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.pointer;
        for &col in &DEFAULT_ORDER[i..] {
            self.pointer += 1;
            let play_pos = Board::col_to_pos(self.possible, col);
            if Board::valid_play_pos(play_pos) {
                return Some((play_pos, col))
            }
        }

        None
    }
}
