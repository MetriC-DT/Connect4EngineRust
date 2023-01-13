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

use std::path::Path;
use crate::board::PLAYABLE_REGION;
use crate::board::{Board, Position};
use crate::nnue::Nnue;

/// Evaluator for a position. Used to obtain a move score for move sorting.
pub trait Evaluator {
    /// creates a new evaluator.
    fn new() -> Self;

    /// calculates the score of a position if a player decides to play `mv`.
    fn eval(&mut self, board: &Board, mv: Position) -> i8;
}


/// Evaluator that calculates the score of a move based on the number of threats it can create.
pub struct ThreatCountEvaluator {}


/// Evaluator that calculates the score of a move based on neural network.
pub struct NnueEvaluator {
    nnue: Nnue,
}


impl Evaluator for ThreatCountEvaluator {
    fn new() -> Self {
        Self { }
    }

    /// counts the number of threats we have, if we played mv.
    fn eval(&mut self, board: &Board, mv: Position) -> i8 {
        let player = board.get_curr_player_pos();
        let total_board = board.get_total_pos();
        let not_taken = PLAYABLE_REGION ^ total_board;
        let winning_position = Board::winning_moves(player | mv, not_taken);
        winning_position.count_ones() as i8
    }
}


impl Evaluator for NnueEvaluator {
    fn new() -> Self {
        let modelpath = Path::new("nnue/export_model.pth");
        let nnue = Nnue::new(modelpath, tch::Device::Cpu).unwrap();
        Self { nnue }
    }

    fn eval(&mut self, board: &Board, mv: Position) -> i8 {
        // we need to "pretend" that the player made the move already.
        let opp_player = board.get_curr_player_pos() | mv;
        let curr_player = board.get_opp_player_pos();

        let moves = board.moves_played() + 1;
        let p2mv = (moves % 2) as u8;

        let (p0, p1) = if p2mv == 0 {
            (curr_player, opp_player)
        } else {
            (opp_player, curr_player)
        };

        // we return the negative score since if this position after the player played `mv` is bad,
        // that means the move must have been good for the player who made the move.
        -self.nnue.evaluate(p0, p1, p2mv, moves)
    }
}
