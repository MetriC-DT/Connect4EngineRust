use crate::transpositiontable::{TranspositionTable, FLAG_UPPER, FLAG_LOWER};
use crate::moves::EMPTY_MOVE;
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

    pub fn add_mv(&mut self, mv: u8) -> Result<(), &str> {
        self.board.add(mv)
    }

    fn play(&mut self, mv: Position) {
        self.board.play(mv);
        self.moves_played += 1;
    }

    fn revert(&mut self, board: Board) {
        self.board = board;
        self.moves_played -= 1;
    }

    /// if we do not need the move, it is much faster to just call the get_eval function.
    pub fn get_mv_eval(&mut self) -> (u8, i8) {
        if let Some(eval) = self.game_over_eval() {
            return (EMPTY_MOVE, -eval);
        }

        let board_clone = self.board;
        let mut move_scores = Vec::new();

        for mv in self.board.get_valid_moves() {
            self.play(mv);
            let eval = -self.get_eval();
            move_scores.push((mv, eval));
            self.revert(board_clone);
        }

        move_scores.sort_unstable_by_key(|(_mv, eval)| { *eval });
        let &(mv, score) = move_scores.last().unwrap();
        
        (Board::pos_to_col(mv), score)
    }

    /// returns the optimal move and evaluation for this explorer's current position.
    pub fn get_eval(&mut self) -> i8 {
        // TODO - check if move is in openings database.

        // Checks if the game is already over.
        if let Some(eval) = self.game_over_eval() {
            return -eval;
        }

        // game is guaranteed to not be over. Therefore, we need to search.
        // Since our score is calculated with best_score = MAX_SCORE - moves_played,
        // we can use these bounds as our (a, b) window.
        // Somehow, widening the window performs better.
        let starter: i8 = MAX_SCORE - self.board.moves_played() as i8;
        let (mut min, mut max) = (-starter, starter);
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

    /// Searches for the most optimal evaluation and move with the given position.
    /// Applies these optimizations:
    /// * alpha-beta pruning
    /// * negamax (principal variation search)
    /// * transposition table
    fn search(&mut self,
              mut a: i8,
              mut b: i8) -> i8 {

        // increment nodes searched.
        self.nodes_explored += 1;

        let board_cpy = self.board;

        // quick endgame lookahead. checks if game ends in one move.
        for mv in self.board.get_valid_moves() {
            self.play(mv);

            if let Some(val) = self.game_over_eval() {
                // README: Returning val instantly like this only works when
                // the the player cannot hope to play another move that ends
                // the game with a better result. For connect4, on the same move,
                // the player cannot have a move that results in a draw and another
                // that results in him winning. Therefore, the best and only move that
                // ends the game right away is the current one.
                self.revert(board_cpy);
                return val;
            }
            // restore original board.
            self.revert(board_cpy);
        }

        b = i8::min(b, MAX_SCORE - self.moves_played as i8);
        if a >= b {
            return b;
        }

        // the index to insert into the principal variation.
        // let pv_index = board.moves_played() as usize;

        // look up evaluation in transposition table
        let board_key = self.board.get_unique_position_key();
        if let Some(entry) = self.transpositiontable.get_entry_with_key(board_key) {
            let flag = entry.get_flag();
            let val = entry.get_eval();

            // if flag == FLAG_EXACT {
            //     return (mv, val);
            // }
            if flag == FLAG_LOWER { a = i8::max(a, val); }
            else if flag == FLAG_UPPER { b = i8::min(b, val); }

            if a >= b { // CUT node.
                return val;
            }
        }

        let mut val = -MAX_SCORE;
        let mut first = true;
        let a_orig = a;

        // calculate evaluation.
        for m in self.board.get_valid_moves() {
            self.play(m);

            let mut score;
            if first { // if first child, then assume it is the best move. Scan entire window.
                let eval = self.search(-b, -a);
                score = -eval;
                first = false;
            }
            else { // search with a null window.
                let eval = self.search(-a - 1, -a);
                score = -eval;

                if a < score && score < b { // if failed high, do a full re-search.
                    let eval = self.search(-b, -score);
                    score = -eval;
                }
            }

            // revert back to original position
            self.revert(board_cpy);

            if score > val {
                val = score;
                a = i8::max(score, a);
            }

            if a >= b { // fail-high beta cutoff occurred. This is a CUT node.
                self.transpositiontable.insert_with_key(board_key, val, FLAG_LOWER);
                return val;
            }
        }

        // insert into transposition table.
        if val <= a_orig { // fail-low occurred. This is an ALL node.
            self.transpositiontable.insert_with_key(board_key, val, FLAG_UPPER);
        }

        val
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
