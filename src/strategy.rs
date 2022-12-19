use crate::{board::{Board, SIZE}, moves::{MoveEvalPair, EMPTY_MOVE}, transpositiontable::TranspositionTable};

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
    pub board: Board,
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

    pub fn solve(&mut self) -> MoveEvalPair {
        if let Some(eval) = Self::game_over_eval(&self.board) {
            MoveEvalPair::new(EMPTY_MOVE, eval)
        }
        else {
            // game is guaranteed to not be over. Therefore, we are
            // allowed to call negamax_eval_pair.
            let starter = MAX_SCORE + 1;
            let a = -starter;
            let b = starter;

            self.negamax_eval_pair(a, b)
        }
    }


    /// A `negamax` function that also generates a move in addition
    /// to the evaluation of the position.
    ///
    /// ASSUMES the game is not yet over.
    fn negamax_eval_pair(&mut self, mut a: i8, mut b: i8) -> MoveEvalPair {
        // increment nodes searched.
        self.nodes_explored += 1;

        let mut orig_board_copy = self.board.clone();

        // quick endgame lookahead. checks if game ends in one move.
        for col in self.board.get_valid_moves() {
            orig_board_copy.add_unchecked(col);

            if let Some(val) = Explorer::game_over_eval(&orig_board_copy) {
                // README: Returning val instantly like this only works when
                // the the player cannot hope to play another move that ends
                // the game with a better result. For connect4, on the same move,
                // the player cannot have a move that results in a draw and another
                // that results in him winning. Therefore, the best and only move that
                // ends the game right away is the current one.

                // let player_val = val * self.board.get_current_player_signed();
                let player_val = val;
                return MoveEvalPair::new(col, player_val);
            }

            // restore orig_board_copy
            orig_board_copy = self.board;
        }

        // TODO - check if move is in openings database.

        let mut value = -MAX_SCORE;
        let mut mv = EMPTY_MOVE;
        let validmoves = self.board.get_valid_moves();

        // look up in transposition table
        if let Some(val) = self.transpositiontable.get(&self.board) {
            if val > b || val < a { return MoveEvalPair::new(EMPTY_MOVE, val); }
        }

        // evaluation value of position
        for m in validmoves {
            self.board.add_unchecked(m);

            let eval_val = -self.negamax_eval_pair(-b, -a).get_eval();
            if eval_val > value {
                value = eval_val;
                mv = m;
            }
            a = i8::max(a, value);

            // revert back to original position
            self.board = orig_board_copy;

            if a >= b { break; }
        }

        self.transpositiontable.insert(&self.board, value);
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
        else if board.is_filled() {
            Some(TIE_SCORE)
        }

        // otherwise, the game is still ongoing.
        else {
            None
        }
    }

    /// returns the number of nodes explored.
    pub fn get_nodes_explored(&self) -> usize {
        self.nodes_explored
    }
}
