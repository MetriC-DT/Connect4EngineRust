use crate::{board::{Board, SIZE}, moves::MoveEvalPair};

pub const MAX_SCORE: i8 = 50;
pub const TIE_SCORE: i8 = 0;
pub const MIN_DEPTH: i8 = SIZE as i8;

// Evaluation table for number of possible 4-in-a-rows
pub const EVALTABLE: [i16; SIZE as usize] = [
    3, 4, 5,  7,  5,  4, 3,
    4, 6, 8,  10, 8,  6, 4,
    5, 8, 11, 13, 11, 8, 5,
    5, 8, 11, 13, 11, 8, 5,
    4, 6, 8,  10, 8,  6, 4,
    3, 4, 5,  7,  5,  4, 3
];

#[derive(Debug)]
pub struct Explorer {
    pub board: Board,
    nodes_explored: usize,
}

impl Explorer {
    pub fn new() -> Self {
        let board = Board::new();
        let nodes_explored = 0;
        Self { board, nodes_explored }
    }

    pub fn with_board(board: Board) -> Self {
        let nodes_explored = 0;
        Self { board, nodes_explored }
    }

    pub fn change_board(&mut self, board: &Board) {
        self.board = *board;
    }

    pub fn strategy(&mut self) -> MoveEvalPair {
        let alpha = -MAX_SCORE;
        let beta = MAX_SCORE;

        if self.board.get_current_player() == 0 {
            self.negamax(alpha, beta)
        }
        else {
            self.negamax(-beta, -alpha)
        }
    }

    /// TODO - only return evaluation.
    fn negamax(&mut self, mut a: i8, b: i8) -> MoveEvalPair {
        // increment nodes searched.
        self.nodes_explored += 1;

        let mut orig_board_copy = self.board;

        // checks if game ends in one move
        for col in self.board.get_valid_moves() {
            orig_board_copy.add_unchecked(col);
            if let Some(val) = Explorer::game_over_eval(&orig_board_copy) {
                return MoveEvalPair::new(col, val);
            }
            orig_board_copy = self.board;
        }

        // evaluation pair to be returned
        let mut p = MoveEvalPair::new(u8::MAX, i8::MIN);

        // obtains the valid moves
        for m in self.board.get_valid_moves() {
            self.board.add_unchecked(m);
            let pair = self.negamax(-b, -a);

            // revert back to original position
            self.change_board(&orig_board_copy);

            let eval_val = -pair.get_eval();

            if eval_val > p.get_eval() {
                p.set_move(m);
                p.set_eval(eval_val);
            }

            a = i8::max(a, p.get_eval());
            if a >= b {
                break;
            }
        }

        p
    }

    /// returns None if not game over. Otherwise, will
    /// return the evaluation of the board
    pub fn game_over_eval(board: &Board) -> Option<i8> {
        let moves_until_end = SIZE - board.moves_played();
        // if first or second player wins, return the maximum score.
        if board.is_first_player_win() ||
            board.is_second_player_win() {
            Some(MAX_SCORE + moves_until_end as i8)
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
