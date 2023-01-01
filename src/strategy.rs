use crate::scoredmoves::ScoredMoves;
use crate::transpositiontable::{TranspositionTable, FLAG_UPPER, FLAG_LOWER, FLAG_EXACT};
use crate::moves::{EMPTY_MOVE, Moves};
use crate::board::{SIZE, Board};

pub const MAX_SCORE: i8 = 1 + SIZE as i8;
pub const TIE_SCORE: i8 = 0;
pub const PV_SIZE: usize = SIZE as usize;

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
    transpositiontable: TranspositionTable
}

impl Explorer {
    pub fn new() -> Self {
        let nodes_explored = 0;
        let transpositiontable = TranspositionTable::new();
        Self { nodes_explored, transpositiontable }
    }

    /// returns the optimal move and evaluation for this explorer's current position.
    pub fn solve(&mut self, board: &Board) -> (u8, i8) {
        // TODO - check if move is in openings database.

        // needs to clear our transposition table first.
        self.transpositiontable.clear();
        let eval = self.evaluate(board);

        if let Some(entry) = self.transpositiontable.get_non_upper_entry(board) {
            return (entry.get_mv(), eval);
        }

        // TODO - got here. Position probably is winning by next move, or losing by next opponent
        // move since we don't store those values in the transposition table.
        let possible = board.possible_moves();

        // winning case
        let winning_moves = board.player_win_moves(possible);
        if winning_moves != 0 {
            let col = Board::pos_to_col(winning_moves);
            return (col, eval);
        }
        // losing case
        let losing_moves = board.opp_win_moves(possible);
        if losing_moves != 0 {
            let col = Board::pos_to_col(losing_moves);
            return (col, eval);
        }

        panic!("Node not found in transposition table.")
    }

    /// evaluates the position, but doesn't return a corresponding move.
    pub fn evaluate(&mut self, board: &Board) -> i8 {
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

        // -1 and +1 on the ends in order for us to be able to obtain an exact move.
        let (min, max) = (start_min - 1, start_max + 1);

        // let mut eval = 0;

        // we will use the null window to check if our score is higher or lower. We will basically
        // use a binary search to home in on the correct node within the correct narrower window.
        // TODO - this should be subject to change, as we want to scan shallower depths before
        // deeper depths, and shallower ones are closer to either `min` or `max`.
        // while min < max {
        //     let mut med = min + (max - min)/2;
        //     if med <= 0 && min/2 < med {
        //         med = min/2;
        //     }
        //     else if med >= 0 && max/2 > med {
        //         med = max/2;
        //     }

        //     eval = self.search(&board, med, med + 1); // the null window search
        //     if eval <= med {
        //         max = eval;
        //     }
        //     else {
        //         min = eval;
        //     }
        // }

        self.search(board, min, max)
    }

    /// Searches for the most optimal evaluation after loading in a board.
    /// Applies these optimizations:
    /// * alpha-beta pruning
    /// * negamax (principal variation search)
    /// * transposition table
    fn search(&mut self,
              board: &Board,
              mut a: i8,
              mut b: i8) -> i8 {

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

        // if we had lost, it would have been on the turn after the next.
        // if a is less than the minimum possible score we can achieve, we can raise the bounds.
        // TODO - subtract moves by 1 in order to be able to obtain an exact node.
        let min_eval = -Explorer::win_eval(moves_played + 1);
        a = i8::max(a, min_eval);

        // if we had won, it would have been on the next turn.
        // if b is greater than the maximum possible score we can achieve, we can lower the bounds.
        // This gives us additional chances to see if we can prune.
        // TODO - subtract moves by 1 in order to be able to obtain an exact node.
        let max_eval = Explorer::win_eval(moves_played + 2);
        b = i8::min(b, max_eval);

        // prune, as this is a cut node.
        if a >= b {
            return a;
        }

        // if there are more than 1 move that enables opponent to win, we are toast.
        let essential_moves = board.opp_win_moves(possible);
        if essential_moves != 0 && !Board::at_most_one_bit_set(essential_moves) {
            let lose_eval = -Explorer::win_eval(moves_played + 2);
            return lose_eval;
        }

        // look up evaluation in transposition table
        if let Some(entry) = self.transpositiontable.get_entry_with_key(board_key) {
            let flag = entry.get_flag();
            let val = entry.get_eval();

            if flag == FLAG_LOWER { a = i8::max(a, val); }
            else if flag == FLAG_UPPER { b = i8::min(b, val); }
            else { return val; }

            if a >= b { // CUT node.
                return val;
            }
        }

        // We only want to search the essential moves, if there are more than 0.
        let next_moves = if essential_moves != 0 {
            let column = Board::pos_to_col(essential_moves);
            ScoredMoves::new_with(essential_moves, column, i8::MAX)

        } else {
            let mut moves = ScoredMoves::new();
            for (m, c) in Moves::new(possible) {
                moves.add(m, c, board.move_score(m));
            }
            moves
        };

        // for use in principal variation search.
        let mut first = true;
        let mut final_eval = -MAX_SCORE;
        let mut final_mv = EMPTY_MOVE;
        let a_orig = a;

        // calculate evaluation.
        let mut boardcpy = *board;
        for (m, c) in next_moves {
            boardcpy.play(m);

            let mut val;
            if first { // if first child, then assume it is the best move. Scan entire window.
                val = -self.search(&boardcpy, -b, -a);
                first = false;
            }
            else { // search with a null window.
                val = -self.search(&boardcpy, -a - 1, -a);

                if a < val && val < b { // if failed high, do a full re-search.
                    val = -self.search(&boardcpy, -b, -val);
                }
            }

            // revert back to original position
            boardcpy = *board;

            // fail-high beta cutoff occurred. This is a CUT node.
            if val >= b {
                // move inserted is refutation move.
                // can use this inserted move for move ordering.
                self.transpositiontable.insert_with_key(board_key, val, FLAG_LOWER, depth, c);
                return val;
            }

            if val > final_eval {
                a = i8::max(val, a);
                final_eval = val;
                final_mv = c;
            }
        }

        // insert into transposition table.
        let flag = if a > a_orig { // exact node (a < val < b)
            FLAG_EXACT
        } else { // fail-low occurred.
            FLAG_UPPER
        };

        self.transpositiontable.insert_with_key(board_key, final_eval, flag, depth, final_mv);
        final_eval
    }

    /// returns positive number upon winning. 0 for not win.
    fn win_eval(moves_played: u32) -> i8 {
        MAX_SCORE - moves_played as i8
    }

    /// returns the number of nodes explored.
    pub fn get_nodes_explored(&self) -> usize {
        self.nodes_explored
    }
}
