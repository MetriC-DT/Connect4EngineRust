use crate::transpositiontable::{TranspositionTable, FLAG_UPPER, FLAG_EXACT, FLAG_LOWER};
use crate::moves::EMPTY_MOVE;
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
    /// position to solve.
    board: Board,

    /// number of nodes this explorer has searched.
    nodes_explored: usize,

    /// Principle variation of the board (updated on `search` function).
    /// TODO - IMPLEMENT. Maybe triangular PV table.
    pv: [u8; PV_SIZE],

    /// transposition table used by the explorer.
    transpositiontable: TranspositionTable
}

impl Explorer {
    pub fn new() -> Self {
        let board = Board::new();
        let nodes_explored = 0;
        let transpositiontable = TranspositionTable::new();
        let pv = [EMPTY_MOVE; PV_SIZE];
        Self { board, pv, nodes_explored, transpositiontable }
    }

    pub fn with_board(board: Board) -> Self {
        let nodes_explored = 0;
        let transpositiontable = TranspositionTable::new();
        let pv = [EMPTY_MOVE; PV_SIZE];
        Self { board, pv, nodes_explored, transpositiontable }
    }

    pub fn change_board(&mut self, board: &Board) {
        let pv = [EMPTY_MOVE; PV_SIZE];
        self.board = *board;
        self.pv = pv;
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn add_mv(&mut self, mv: u8) -> Result<(), &str> {
        self.board.add(mv)
    }

    pub fn get_pv(&self) -> &[u8] {
        let first = self.get_board().moves_played() as usize;
        let mut end: usize = first;
        for i in first..PV_SIZE {
            if self.pv[end] == EMPTY_MOVE { break; }
            else { end = i }
        }

        &self.pv[first..=end]
    }

    /// returns the optimal move and evaluation for this explorer's current position.
    pub fn solve(&mut self) -> (u8, i8) {
        // TODO - check if move is in openings database.

        // Checks if the game is already over.
        if let Some(eval) = Self::game_over_eval(&self.board) {
            (EMPTY_MOVE, eval);
        }

        // game is guaranteed to not be over. Therefore, we need to search.
        let board_clone = self.board;

        // Since our score is calculated with best_score = MAX_SCORE - moves_played,
        // we can use these bounds as our (a, b) window.
        let starter: i8 = MAX_SCORE - self.board.moves_played() as i8;
        let (min, max) = (-starter, starter + 1);

        // we will use the null window to check if our score is higher or lower. We will basically
        // use a binary search to home in on the correct node within the correct narrower window.
        let (_mv, eval) = self.search(board_clone, 0, 1);

        if eval > 0 {
            self.search(board_clone, 0, max)
        } else {
            self.search(board_clone, min, 0)
        }
    }

    /// Searches for the most optimal evaluation and move with the given position.
    /// Applies these optimizations:
    /// * alpha-beta pruning
    /// * negamax (principal variation search)
    /// * transposition table
    fn search(&mut self,
              board: Board,
              mut a: i8,
              mut b: i8) -> (u8, i8) {

        // increment nodes searched.
        self.nodes_explored += 1;

        // if game has ended, return evaluation.
        // if let Some(eval) = Self::game_over_eval(&board) {
        //     return (EMPTY_MOVE, -eval);
        // }

        let mut board_cpy = board;
        // quick endgame lookahead. checks if game ends in one move.
        for col in board.get_valid_moves() {
            board_cpy.add_unchecked(col);
            if let Some(val) = Explorer::game_over_eval(&board_cpy) {
                // README: Returning val instantly like this only works when
                // the the player cannot hope to play another move that ends
                // the game with a better result. For connect4, on the same move,
                // the player cannot have a move that results in a draw and another
                // that results in him winning. Therefore, the best and only move that
                // ends the game right away is the current one.
                // let player_val = val * self.board.get_current_player_signed();
                return (col, val);
            }
            // restore orig_board_copy
            board_cpy = board;
        }

        // the index to insert into the principal variation.
        // let pv_index = board.moves_played() as usize;

        // look up evaluation in transposition table
        let board_key = board.get_unique_position_key();
        if let Some(entry) = self.transpositiontable.get_entry_with_key(board_key) {
            let flag = entry.get_flag();
            let val = entry.get_eval();
            let mv = entry.get_move();

            if flag == FLAG_EXACT {
                return (mv, val);
            }
            else if flag == FLAG_LOWER { a = i8::max(a, val); }
            else if flag == FLAG_UPPER { b = i8::min(b, val); }

            if a >= b { // CUT node.
                return (mv, val);
            }
        }

        let (mut mv, mut val) = (EMPTY_MOVE, -MAX_SCORE);
        let mut first = true;
        let a_orig = a;

        // calculate evaluation.
        for m in board.get_valid_moves() {
            board_cpy.add_unchecked(m);

            let mut score;
            if first { // if first child, then assume it is the best move. Scan entire window.
                let (_, eval) = self.search(board_cpy, -b, -a);
                score = -eval;
                first = false;
            }
            else { // search with a null window.
                let (_, eval) = self.search(board_cpy, -a - 1, -a);
                score = -eval;

                if a < score && score < b { // if failed high, do a full re-search.
                    let (_, eval) = self.search(board_cpy, -b, -score);
                    score = -eval;
                }
            }

            // revert back to original position
            board_cpy = board;

            if score > val {
                (mv, val) = (m, score);
                a = i8::max(score, a);
            }

            if a >= b { break; }
        }

        // insert into transposition table.
        if val <= a_orig { // fail-low occurred. This is an ALL node.
            self.transpositiontable.insert_with_key(board_key, val, FLAG_UPPER, mv);
        } else if a >= b { // fail-high beta cutoff occurred. This is a CUT node.
            // beta cutoff is guaranteed to not be part of the principal variation.
            self.transpositiontable.insert_with_key(board_key, val, FLAG_LOWER, mv);
        } else { // This is the PV node.
            self.transpositiontable.insert_with_key(board_key, val, FLAG_EXACT, mv);
        }
        (mv, val)
    }

    /// returns None if not game over. Otherwise, will
    /// return the evaluation of the board
    pub fn game_over_eval(board: &Board) -> Option<i8> {
        if board.is_player_win() {
            // Added size here so we can select the move that finishes the game 
            // the quickest.
            let score: i8 = MAX_SCORE - board.moves_played() as i8;
            // return Some(board.get_prev_player_signed() * score);
            return Some(score);
        }

        // if draw game
        else if board.is_filled() { Some(TIE_SCORE) }

        // otherwise, the game is still ongoing.
        else { None }
    }

    /// returns the number of nodes explored.
    pub fn get_nodes_explored(&self) -> usize {
        self.nodes_explored
    }
}
