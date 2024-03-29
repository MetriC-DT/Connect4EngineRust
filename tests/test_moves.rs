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

use connect4engine::scoredmoves::ScoredMoves;
use connect4engine::moves::{Moves, DEFAULT_ORDER};
use connect4engine::board::{Board, BOTTOM_ROW_MASK};

#[test]
fn test_moves_ordering() {
    let emptyboard = Board::new();
    let mvs = Moves::new(emptyboard.possible_moves());

    let ordering = [3, 2, 4, 1, 5, 0, 6];
    for ((mv, c), col) in mvs.zip(ordering) {
        let mv_col = Board::pos_to_col(mv);
        assert_eq!(mv_col, col);
        assert_eq!(c, col);
    }
}

#[test]
fn test_scored_moves_ties() {
    // there are ties
    let ordering = DEFAULT_ORDER;
    let scores = [3, 5, 4, 4, 3, 1, 0];
    let expected_order = [2, 4, 1, 3, 5, 0, 6];

    test_scored_moves_ordering(&ordering, &scores, &expected_order);
}

#[test]
fn test_scored_moves_single() {
    // inserts only 1 element into the scored moves array.
    let col = 5;
    let score = 1;
    let mut scored_mv = ScoredMoves::new_with(0, col, score);

    assert_eq!(scored_mv.clone().count(), 1);

    let (check_mv, check_col) = scored_mv.next().unwrap();
    assert_eq!(check_mv, 0);
    assert_eq!(check_col, col);
}

#[test]
fn test_scored_moves_out_of_order() {
    let ordering = DEFAULT_ORDER;
    let scores = [0, 1, 2, 3, 4, 5, 6]; // completely inverted order.
    let expected_order: Vec<u8> = DEFAULT_ORDER.into_iter().rev().collect();
    test_scored_moves_ordering(&ordering, &scores, &expected_order);
}

fn test_scored_moves_ordering(ordering: &[u8], scores: &[i8], expected_order: &[u8]) {
    let mut scored_moves = ScoredMoves::new();

    for (&col, &score) in ordering.iter().zip(scores.iter()) {
        let pos = Board::col_to_pos(BOTTOM_ROW_MASK, col);
        scored_moves.add(pos, col, score);
    }

    let mut count = 0;
    for (m, &expected) in scored_moves.zip(expected_order) {
        let (mv, col) = m;
        assert_eq!(col, expected);
        assert_eq!(Board::pos_to_col(mv), col);
        count += 1;
    }

    assert_eq!(count, scores.len());
}
