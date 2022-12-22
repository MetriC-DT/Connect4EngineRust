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

    pub fn solve(&mut self) -> MoveEvalPair {
        // TODO - check if move is in openings database.
        let starter = MAX_SCORE + 1;
        let a = -starter;
        let b = starter;

        let board_clone = self.board;

        let depth = SIZE - board_clone.moves_played();
        if let Some(pair) = self.negamax_eval_pair(board_clone, depth, a, b) {
            pair
        }
        else {
            MoveEvalPair::new(EMPTY_MOVE, i8::MIN)
        }
    }


    /// A `negamax` function that also generates a move in addition
    /// to the evaluation of the position.
    ///
    /// ASSUMES the game is not yet over.
    fn negamax_eval_pair(&mut self,
                         board: Board,
                         depth: u8,
                         mut a: i8,
                         mut b: i8) -> Option<MoveEvalPair> {

        // increment nodes searched.
        self.nodes_explored += 1;

        if let Some(val) = Self::game_over_eval(&board) {
            return Some(MoveEvalPair::new(EMPTY_MOVE, -val));
        }

        // look up in transposition table
        let board_key = board.get_unique_position_key();
        if let Some(entry) = self.transpositiontable.get_entry_with_key(board_key) {
            let flag = entry.get_flag();
            let val = entry.get_eval();
            let mv = entry.get_move();

            if      flag == FLAG_EXACT { return Some(MoveEvalPair::new(mv, val)); }
            else if flag == FLAG_LOWER { a = i8::max(a, val); }
            else if flag == FLAG_UPPER { b = i8::min(b, val); }

            if a >= b {
                return Some(MoveEvalPair::new(mv, val));
            }
        }

        if depth == 0 {
            return None;
        }

        let mut value = -MAX_SCORE;
        let mut mv = EMPTY_MOVE;
        let a_orig = a;
        let mut board_cpy = board.clone();

        // evaluation value of position
        for m in board.get_valid_moves() {
            board_cpy.add_unchecked(m);

            let found = self.negamax_eval_pair(board_cpy, depth - 1, -b, -a);

            if let Some(pair) = found {
                let eval_val = -pair.get_eval();
                if eval_val > value {
                    value = eval_val;
                    mv = m;
                    a = i8::max(a, value);
                }
            }

            if a >= b {
                break;
            }
            else {
                // revert back to original position
                board_cpy = board;
            }
        }

        // insert into transposition table.
        if value <= a_orig {
            self.transpositiontable.insert_with_key(board_key, value, FLAG_UPPER, mv);
        } else if value >= b {
            self.transpositiontable.insert_with_key(board_key, value, FLAG_LOWER, mv);
        } else {
            self.transpositiontable.insert_with_key(board_key, value, FLAG_EXACT, mv);
        }

        Some(MoveEvalPair::new(mv, value))
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
