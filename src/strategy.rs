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

use crate::scoredmoves::ScoredMoves;
use crate::transpositiontable::{TranspositionTable, FLAG_UPPER, FLAG_LOWER, FLAG_EXACT};
use crate::moves::{EMPTY_MOVE, Moves};
use crate::board::{SIZE, Board, Position};

pub const MAX_SCORE: i8 = 2 + SIZE as i8;
pub const TIE_SCORE: i8 = 0;
pub const PV_SIZE: usize = SIZE as usize;
const REFUTATION_SCORE: i8 = 16;

// Evaluation table for number of possible 4-in-a-rows
/*
pub const EVALTABLE: [i16; SIZE as usize] = [
    3, 4, 5,  7,  5,  4, 3,
    4, 6, 8,  10, 8,  6, 4,
    5, 8, 11, 13, 11, 8, 5,
    5, 8, 11, 13, 11, 8, 5,
    4, 6, 8,  10, 8,  6, 4,
    3, 4, 5,  7,  5,  4, 3
];
*/

#[derive(Debug)]
pub struct Explorer {
    /// number of nodes this explorer has searched.
    nodes_explored: usize,

    /// transposition table used by the explorer.
    transpositiontable: TranspositionTable,

    /// number of moves that failed low.
    fail_low_nodes: usize,

    /// number of moves that failed high.
    fail_high_nodes: usize
}

impl Explorer {
    pub fn new() -> Self {
        let nodes_explored = 0;
        let transpositiontable = TranspositionTable::new();
        let (fail_low_nodes, fail_high_nodes) = (0, 0);
        Self { nodes_explored, transpositiontable, fail_low_nodes, fail_high_nodes }
    }

    /// returns the optimal move and evaluation for this explorer's current position.
    pub fn solve(&mut self, board: &Board) -> (u8, i8) {
        // TODO - check if move is in openings database.

        // needs to clear our transposition table first. Otherwise, we might store some nodes that
        // failed low, which are unusable for finding the principal variation.
        let eval = self.evaluate_board(board, true);

        // if the game is already over, then we can't play anything.
        if board.is_game_over() {
            return (EMPTY_MOVE, eval);
        }

        let pv = self.get_pv(board);
        // println!("{:?}", pv);

        if !pv.is_empty() {
            return (pv[0], eval);
        }

        panic!("Node not found in transposition table.")
    }

    fn get_pv(&self, board: &Board) -> Vec<u8> {
        let mut pv = Vec::new();
        let mut board_cpy = *board;

        loop {
            // TODO checks if game over.
            if let Some(entry) = self.transpositiontable.get_exact_entry(&board_cpy) {
                let mv = entry.get_mv();
                if board_cpy.add(mv).is_ok() {
                    pv.push(mv);
                    continue;
                }
            }

            break;
        }

        // Position probably is winning by next move, or losing by next opponent
        // move since we don't store those values in the transposition table.
        let possible = board_cpy.possible_moves();

        // winning case
        let winning_moves = board_cpy.player_win_moves(possible);
        if winning_moves != 0 {
            let col = Board::pos_to_col(winning_moves);
            pv.push(col);
        }
        // losing case: TODO - needs to be fixed to give the longest line.
        let (losing_moves, _) = board_cpy.opp_win_moves(possible);
        if losing_moves != 0 {
            let col = Board::pos_to_col(losing_moves);
            pv.push(col);
        }
        // draw case
        if possible != 0 {
            let col = Board::pos_to_col(possible);
            pv.push(col);
        }

        pv
    }

    pub fn evaluate(&mut self, board: &Board) -> i8 {
        self.evaluate_board(board, false)
    }

    /// evaluates the position, but doesn't return a corresponding move.
    pub fn evaluate_board(&mut self, board: &Board, reset_t_table: bool) -> i8 {

        // Checks if the game is already over.
        if board.has_winner() {
            // if board has winner already, then assume the current player is loser.
            // Therefore, the score would be negative.
            return -Explorer::win_eval(board.moves_played());
        }
        else if board.is_filled() {
            return TIE_SCORE;
        }

        // game is guaranteed to not be over. Therefore, we need to search.
        // Since our score is calculated with best_score = MAX_SCORE - moves_played,
        // we can use these bounds as our (a, b) window.

        // the maximum score we can get is when we win directly on our next move.
        let start_max: i8 = Explorer::win_eval(board.moves_played() + 1);
        let start_max: i8 = i8::min(start_max, Explorer::win_eval(7)); // fastest win on 7 moves.

        // the minimum score we can get is when we lose on the opponent's move (2 more moves).
        let start_min: i8 = -Explorer::win_eval(board.moves_played() + 2);
        let start_min: i8 = i8::max(start_min, -Explorer::win_eval(8)); // fastest loss on 8 moves.


        // our principal variation is guaranteed to be evaluated within the bounds (min, max).
        // However, we can have an aspiration window to reduce the number of nodes to search.
        //
        // We are finished if we find an evaluation in the non-inclusive range (min, max).
        // Searching at lower windows (e.g. nearer to min) results in less fail-lows, which means
        // that we get a conclusive result (whether the evaluation was found or not) with fewer
        // nodes scanned. Additionally, windows near the extremes of range (min, max) are also at
        // comparatively lower depths relative to windows nearer to the center, due to the way our
        // winning scores are assigned (e.g. MAX_SCORE - moves_played).

        if board.moves_played() <= 15 { // only use the aspiration window if depth is past certain threshold.
            let (mut g_min, mut g_max) = (start_min, start_max);
            let low_sz = 6;
            let high_sz = 6;
            let (mut min, mut max) = (g_min, g_min + low_sz);

            loop {
                // -1 and +1 on the bounds in order for us to be able to obtain an exact move.
                let asp_min = i8::max(min - 1, g_min - 1);
                let asp_max = i8::min(max + 1, g_max + 1);

                if reset_t_table { self.transpositiontable.clear(); }
                let eval = self.search(board, asp_min, asp_max, Board::move_score);

                if asp_min < eval && eval < asp_max {
                    return eval;
                }
                else if eval <= asp_min { // failed low.
                    g_max = if eval == asp_min { asp_min } else { asp_min - 1 };
                    max = g_min + low_sz;
                    min = g_min;
                }
                else if eval >= asp_max { // failed high.
                    g_min = if eval == asp_max { asp_max } else { asp_max + 1 };
                    min = g_max - high_sz;
                    max = g_max;
                }

                if g_max <= g_min {
                    return eval;
                }
            }
        }
        else {
            if reset_t_table { self.transpositiontable.clear(); }
            return self.search(board, start_min - 1, start_max + 1, Board::move_score);
        }
    }

    /// Searches for the most optimal evaluation after loading in a board.
    /// Applies these optimizations:
    /// * alpha-beta pruning
    /// * negamax (principal variation search)
    /// * transposition table
    /// * Fail-soft boundaries
    ///
    /// Parameters:
    /// board - the board to find the best move of
    /// a - alpha-beta pruning lower bound
    /// b - alpha-beta pruning upper bound
    /// f - move-ordering function (returns score, where higher is better for current player).
    fn search(&mut self,
              board: &Board,
              mut a: i8,
              mut b: i8,
              f: fn(&Board, Position) -> i8) -> i8 {

        // increment nodes searched.
        self.nodes_explored += 1;

        if board.is_filled() { // the position is drawn.
            // we do not need to check if move is win, because winning is already checked before
            // the recursive call (via endgame lookahead).
            return TIE_SCORE;
        }

        let possible = board.possible_moves();
        let winning_moves = board.player_win_moves(possible);
        let moves_played = board.moves_played();

        // the unique key to represent the board in order to insert or search transposition table.
        let board_key = board.get_unique_position_key();
        let depth = moves_played as u8;

        // quick endgame lookahead. checks if can win in 1 move.
        if winning_moves != 0 {
            let win_eval = Explorer::win_eval(moves_played + 1);
            return win_eval;
        }

        // looks for possible moves that don't lose the game immediately.
        let non_losing_moves = board.non_losing_moves(possible);
        if non_losing_moves == 0 { // all moves will lose.
            let lose_eval = -Explorer::win_eval(moves_played + 2);
            return lose_eval;
        }

        // if we had lost, it would have been on 4 turns later (us, opp, us, opp)
        // if a is less than the minimum possible score we can achieve, we can raise the bounds.
        let min_eval = -Explorer::win_eval(moves_played + 3);
        a = i8::max(a, min_eval);

        // if we had won, it would have been on 3 turns later (us, opp, us).
        // if b is greater than the maximum possible score we can achieve, we can lower the bounds.
        // This gives us additional chances to see if we can prune.
        let max_eval = Explorer::win_eval(moves_played + 2);
        b = i8::min(b, max_eval);

        // prune, as this is a cut node.
        if a >= b {
            return a;
        }

        // look up evaluation in transposition table. Updates the best refutation.
        let mut refutation = EMPTY_MOVE;
        if let Some(entry) = self.transpositiontable.get_entry_with_key(board_key) {
            let flag = entry.get_flag();
            let val = entry.get_eval();

            if flag == FLAG_UPPER { // Failed low.
                b = i8::min(b, val);
            }
            else if flag == FLAG_LOWER { // Failed high. We can update refutation move.
                a = i8::max(a, val);
                refutation = entry.get_mv();
            }
            else { // exact node.
                return val;
            }

            if a >= b { // CUT node.
                return val;
            }
        }

        // generates ordered moves to search.
        let mut next_moves = ScoredMoves::new();
        for (m, c) in Moves::new(non_losing_moves) {
            if refutation == c { // prioritize searching refutation move first.
                next_moves.add(m, c, REFUTATION_SCORE);
            }
            else {
                next_moves.add(m, c, f(board, m));
            }
        }

        // for use in principal variation search.
        let mut final_eval = -MAX_SCORE;
        let mut final_mv = EMPTY_MOVE;
        let mut boardcpy = *board;
        let a_orig = a;

        // calculate evaluation.
        for (i, (m, c)) in next_moves.enumerate() {
            boardcpy.play(m);

            let mut val;
            if i == 0 { // if first child, then assume it is the best move. Scan entire window.
                val = -self.search(&boardcpy, -b, -a, f);
            }
            else { // search with a null window.
                val = -self.search(&boardcpy, -a - 1, -a, f);

                if a < val && val < b { // if failed high, do a full re-search.
                    val = -self.search(&boardcpy, -b, -val, f);
                }
            }

            // fail-high beta cutoff occurred. This is a CUT node.
            if val >= b {
                // move inserted is refutation move.
                // can use this inserted move for move ordering.
                self.transpositiontable.insert_with_key(board_key, val, FLAG_LOWER, depth, c);
                self.fail_high_nodes += 1;
                return val;
            }

            if val > final_eval {
                a = i8::max(val, a);
                final_eval = val;
                final_mv = c;
            }

            // revert back to original position
            boardcpy.revert(m);
        }

        // insert into transposition table.
        let flag = if a > a_orig { // exact node (a < val < b)
            FLAG_EXACT
        } else { // fail-low occurred. We cannot use this move.
            self.fail_low_nodes += 1;
            FLAG_UPPER
        };

        self.transpositiontable.insert_with_key(board_key, final_eval, flag, depth, final_mv);
        final_eval
    }

    /// Assumes the game finished in `moves_played` number of moves, and assigns a score to the
    /// winner.
    fn win_eval(moves_played: u32) -> i8 {
        MAX_SCORE - moves_played as i8
    }

    /// returns the number of nodes explored.
    pub fn get_nodes_explored(&self) -> usize {
        self.nodes_explored
    }

    /// percent of nodes that failed low (decimal).
    pub fn get_fail_lows(&self) -> f32 {
        self.fail_low_nodes as f32 / self.nodes_explored as f32
    }

    /// percent of nodes that failed high (decimal).
    pub fn get_fail_highs(&self) -> f32 {
        self.fail_high_nodes as f32 / self.nodes_explored as f32
    }
}
