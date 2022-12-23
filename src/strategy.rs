use crate::transpositiontable::{TranspositionTable, FLAG_UPPER, FLAG_EXACT, FLAG_LOWER};
use crate::moves::{MoveEvalPair, EMPTY_MOVE};
use crate::board::{SIZE, Board};

pub const MAX_SCORE: i8 = 1 + SIZE as i8;
pub const TIE_SCORE: i8 = 0;

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
    board: Board,
    nodes_explored: usize,
    transpositiontable: TranspositionTable
}

impl Explorer {
    pub fn new() -> Self {
        let board = Board::new();
        let nodes_explored = 0;
        let transpositiontable = TranspositionTable::new();
        Self { board, nodes_explored, transpositiontable }
    }

    pub fn with_board(board: Board) -> Self {
        let nodes_explored = 0;
        let transpositiontable = TranspositionTable::new();
        Self { board, nodes_explored, transpositiontable }
    }

    pub fn change_board(&mut self, board: &Board) {
        self.board = *board;
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    /// returns the optimal move and evaluation for this explorer's current position.
    pub fn solve(&mut self) -> MoveEvalPair {
        // TODO - check if move is in openings database.

        // Checks if the game is already over.
        if let Some(eval) = Self::game_over_eval(&self.board) {
            return MoveEvalPair::new(EMPTY_MOVE, eval);
        }

        // game is guaranteed to not be over. Therefore, we need to search.
        let board_clone = self.board;
        let starter = MAX_SCORE + 1;
        let a = -starter;
        let b = starter;

        self.search(board_clone, a, b)
    }


    /// Searches for the most optimal evaluation and move with the given position.
    /// Applies these optimizations:
    /// * alpha-beta pruning
    /// * negamax (principal variation search)
    /// * transposition table
    ///
    /// ASSUMES the game is not yet over.
    fn search(&mut self,
              board: Board,
              mut a: i8,
              mut b: i8) -> MoveEvalPair {

        // increment nodes searched.
        self.nodes_explored += 1;

        // if game has ended, return evaluation.
        if let Some(eval) = Self::game_over_eval(&board) {
            return MoveEvalPair::new(EMPTY_MOVE, -eval);
        }

        // look up evaluation in transposition table
        let board_key = board.get_unique_position_key();
        if let Some(entry) = self.transpositiontable.get_entry_with_key(board_key) {
            let flag = entry.get_flag();
            let val = entry.get_eval();
            let mv = entry.get_move();

            if flag == FLAG_EXACT { return MoveEvalPair::new(mv, val); }
            else if flag == FLAG_LOWER { a = i8::max(a, val); }
            else if flag == FLAG_UPPER { b = i8::min(b, val); }

            if a >= b {
                return MoveEvalPair::new(mv, val);
            }
        }

        let mut found_pair = MoveEvalPair::new(EMPTY_MOVE, -MAX_SCORE);
        let a_orig = a;
        let mut board_cpy = board;
        let mut first = true;

        // calculate evaluation. We need i to determine the first child.
        for m in board.get_valid_moves() {
            board_cpy.add_unchecked(m);

            let mut score;
            if first { // if first child, then assume it is the best move. Scan entire window.
                score = -self.search(board_cpy, -b, -a).get_eval();
                first = false;
            }
            else { // search with a null window.
                score = -self.search(board_cpy, -a - 1, -a).get_eval();
                if a < score && score < b { // if failed high, do a full re-search.
                    score = -self.search(board_cpy, -b, -score).get_eval();
                }
            }

            if score > found_pair.get_eval() {
                found_pair.set(m, score);
                a = i8::max(a, score);
            }

            // revert back to original position
            board_cpy = board;

            if a >= b { break; }
        }

        // insert into transposition table.
        let (mv, value) = found_pair.get_pair();
        if value <= a_orig {
            self.transpositiontable.insert_with_key(board_key, value, FLAG_UPPER, mv);
        } else if value >= b {
            self.transpositiontable.insert_with_key(board_key, value, FLAG_LOWER, mv);
        } else {
            self.transpositiontable.insert_with_key(board_key, value, FLAG_EXACT, mv);
        }

        MoveEvalPair::new(mv, value)
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
