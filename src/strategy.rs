use crate::{board::{Board, SIZE}, moves::MoveEvalPair, transpositiontable::TranspositionTable};

pub const MAX_SCORE: i8 = SIZE as i8;
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

    pub fn solve(&mut self) -> Option<MoveEvalPair> {
        // if given game is already over, do not run search
        if self.board.is_game_over() {
            return None;
        }

        // game is guaranteed to not be over. Therefore, we are
        // allowed to call negamax_eval_pair.
        let a = -MAX_SCORE;
        let b = MAX_SCORE;

        Some(self.negamax_eval_pair(a, b))
    }


    /// A `negamax` function that also generates a move in addition
    /// to the evaluation of the position.
    ///
    /// ASSUMES the game is not yet over.
    fn negamax_eval_pair(&mut self, mut a: i8, b: i8) -> MoveEvalPair {
        // increment nodes searched.
        self.nodes_explored += 1;

        let mut orig_board_copy = self.board;

        // checks if game ends in one move
        for col in self.board.get_valid_moves() {
            orig_board_copy.add_unchecked(col);

            if let Some(val) = Explorer::game_over_eval(&orig_board_copy) {
                // README: Returning val instantly like this only works when
                // the the player cannot hope to play another move that ends
                // the game with a better result. For connect4, on the same move,
                // the player cannot have a move that results in a draw and another
                // that results in him winning. Therefore, the best and only move that
                // ends the game right away is the current one.
                return MoveEvalPair::new(col, val);
            }
            orig_board_copy = self.board;
        }

        // evaluation value of a position
        let mut value = i8::MIN;
        let mut mv = u8::MAX;

        for m in self.board.get_valid_moves() {
            self.board.add_unchecked(m);

            let eval_val;
            if let Some(eval) = self.transpositiontable.get(&self.board) {
                eval_val = eval;
            }
            else {
                eval_val = -self.negamax_eval_pair(-b, -a).get_eval();
                self.transpositiontable.insert(&self.board, eval_val);
            }

            // revert back to original position
            self.change_board(&orig_board_copy);

            if eval_val > value {
                value = eval_val;
                mv = m;
            }

            a = i8::max(a, value);
            if a >= b {
                break;
            }
        }

        MoveEvalPair::new(mv, value)
    }

    /// returns None if not game over. Otherwise, will
    /// return the evaluation of the board
    pub fn game_over_eval(board: &Board) -> Option<i8> {
        // if first or second player wins, return the maximum score.
        if board.is_second_player_win() ||
            board.is_first_player_win() {

            // Added size here so we can select the move that finishes the game 
            // the quickest. score >= 0 so that we can exceed the MAX_SCORE limit.
            let score = SIZE - board.moves_played();
            Some(MAX_SCORE + score as i8)
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
