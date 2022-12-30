use crate::transpositiontable::{TranspositionTable, FLAG_UPPER, FLAG_LOWER};
use crate::moves::{EMPTY_MOVE, Moves};
use crate::board::{SIZE, Board, Position};

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
    /// position to solve.
    board: Board,

    /// moves that have been made on the board.
    moves_played: u32,

    /// number of nodes this explorer has searched.
    nodes_explored: usize,

    /// transposition table used by the explorer.
    transpositiontable: TranspositionTable
}

impl Explorer {
    pub fn new() -> Self {
        let board = Board::new();
        let nodes_explored = 0;
        let transpositiontable = TranspositionTable::new();
        let moves_played = board.moves_played();
        Self { board, moves_played, nodes_explored, transpositiontable }
    }

    pub fn with_board(board: Board) -> Self {
        let nodes_explored = 0;
        let transpositiontable = TranspositionTable::new();
        let moves_played = board.moves_played();
        Self { board, moves_played, nodes_explored, transpositiontable }
    }

    pub fn change_board(&mut self, board: &Board) {
        self.board = *board;
        self.moves_played = board.moves_played();
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    fn play(&mut self, mv: Position) {
        self.board.play(mv);
        self.moves_played += 1;
    }

    fn revert(&mut self, mv: Position) {
        self.board.revert(mv);
        self.moves_played -= 1;
    }

    /// returns the optimal move and evaluation for this explorer's current position.
    pub fn solve(&mut self) -> (u8, i8) {
        // TODO - check if move is in openings database.

        // Checks if the game is already over.
        if let Some(eval) = self.game_over_eval() {
            return (EMPTY_MOVE, -eval);
        }

        let possible = self.get_board().possible_moves();
        let mut pairs = Vec::new();

        for (mv, _c) in Moves::new(possible) {
            self.play(mv);
            let col = Board::pos_to_col(mv);
            if self.board.has_winner() {
                let eval = Explorer::win_eval(self.moves_played);
                self.revert(mv);
                return (col, eval);
            } else {
                let eval = -self.evaluate();
                pairs.push((mv, eval));
            }
            self.revert(mv);
        }

        let (mv, eval) = pairs.into_iter().max_by_key(|&(_m, v)| { v }).unwrap();
        (Board::pos_to_col(mv), eval)
    }

    /// evaluates the position, but doesn't return a corresponding move.
    pub fn evaluate(&mut self) -> i8 {
        // game is guaranteed to not be over. Therefore, we need to search.
        // Since our score is calculated with best_score = MAX_SCORE - moves_played,
        // we can use these bounds as our (a, b) window.

        // the maximum score we can get is when we win directly on our next move.
        let start_max: i8 = Explorer::win_eval(self.moves_played);
        let start_max: i8 = i8::min(start_max, Explorer::win_eval(7)); // fastest win on 7 moves.

        // the minimum score we can get is when we lose on the opponent's move (2 more moves).
        let start_min: i8 = -Explorer::win_eval(self.moves_played);
        let start_min: i8 = i8::max(start_min, -Explorer::win_eval(8)); // fastest loss on 8 moves.

        let (mut min, mut max) = (start_min, start_max);
        let mut eval = 0;

        // we will use the null window to check if our score is higher or lower. We will basically
        // use a binary search to home in on the correct node within the correct narrower window.
        while min < max {
            let mut med = min + (max - min)/2;
            if med <= 0 && min/2 < med {
                med = min/2;
            }
            else if med >= 0 && max/2 > med {
                med = max/2;
            }

            eval = self.search(med, med + 1); // the null window search
            if eval <= med {
                max = eval;
            }
            else {
                min = eval;
            }
        }

        eval
    }

    /// Searches for the most optimal evaluation after loading in a board.
    /// Applies these optimizations:
    /// * alpha-beta pruning
    /// * negamax (principal variation search)
    /// * transposition table
    fn search(&mut self,
              mut a: i8,
              mut b: i8) -> i8 {

        // increment nodes searched.
        self.nodes_explored += 1;

        if self.board.is_filled() { // the position is drawn.
            // we do not need to check if move is win, because winning is already checked before
            // the recursive call (via endgame lookahead).
            return 0;
        }

        // if we had lost, it would have been on the turn after the next.
        // if a is less than the minimum possible score we can achieve, we can raise the bounds.
        let min_eval = -Explorer::win_eval(self.moves_played + 2);
        a = i8::max(a, min_eval);
        if a >= b {
            return a;
        }

        // if we had won, it would have been on the next turn.
        // if b is greater than the maximum possible score we can achieve, we can lower the bounds.
        // This gives us additional chances to see if we can prune.
        let max_eval = Explorer::win_eval(self.moves_played + 1);
        b = i8::min(b, max_eval);
        if a >= b {
            return b;
        }

        let possible = self.board.possible_moves();
        let winning_moves = self.board.player_win_moves(possible);

        // quick endgame lookahead. checks if can win in 1 move.
        if winning_moves != 0 {
            return max_eval;
        }

        // if there are more than 1 move that enables opponent to win, we are toast.
        let essential_moves = self.board.opp_win_moves(possible);
        if essential_moves != 0 && !Board::at_most_one_bit_set(essential_moves) {
            return min_eval;
        }

        // the unique key to represent the board in order to insert or search transposition table.
        let board_key = self.board.get_unique_position_key();

        // look up evaluation in transposition table
        if let Some(entry) = self.transpositiontable.get_entry_with_key(board_key) {
            let flag = entry.get_flag();
            let val = entry.get_eval();

            if flag == FLAG_LOWER { a = i8::max(a, val); }
            else if flag == FLAG_UPPER { b = i8::min(b, val); }

            if a >= b { // CUT node.
                return val;
            }
        }

        // for use in principal variation search.
        let mut first = true;

        // We only want to search the essential moves, if there are more than 0.
        let next_moves = if essential_moves == 0 {
            Moves::new(possible)
        } else {
            Moves::new(essential_moves)
        };

        // calculate evaluation.
        for (m, _) in next_moves {
            self.play(m);

            let mut val;
            if first { // if first child, then assume it is the best move. Scan entire window.
                val = -self.search(-b, -a);
                first = false;
            }
            else { // search with a null window.
                val = -self.search(-a - 1, -a);

                if a < val && val < b { // if failed high, do a full re-search.
                    val = -self.search(-b, -val);
                }
            }

            // revert back to original position
            self.revert(m);

            // fail-high beta cutoff occurred. This is a CUT node.
            if val >= b {
                self.transpositiontable.insert_with_key(board_key, val, FLAG_LOWER);
                return val;
            }

            a = i8::max(val, a);
        }

        // insert into transposition table.
        // fail-low occurred.
        self.transpositiontable.insert_with_key(board_key, a, FLAG_UPPER);
        a
    }

    /// returns positive number upon winning. 0 for not win.
    fn win_eval(moves_played: u32) -> i8 {
        MAX_SCORE - moves_played as i8
    }

    /// returns None if not game over. Otherwise, will
    /// return the evaluation of the board
    pub fn game_over_eval(&self) -> Option<i8> {
        if self.board.has_winner() {
            // Added size here so we can select the move that finishes the game 
            // the quickest.
            let score: i8 = MAX_SCORE - self.moves_played as i8;
            Some(score)
        }

        // if draw game
        else if self.board.is_filled() { Some(TIE_SCORE) }

        // otherwise, the game is still ongoing.
        else { None }
    }

    /// returns the number of nodes explored.
    pub fn get_nodes_explored(&self) -> usize {
        self.nodes_explored
    }
}
