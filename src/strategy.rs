use crate::board::{Board, SIZE, WIDTH, HEIGHT};

pub const MAX_SCORE: i16 = 25000;
pub const MIN_DEPTH: u8 = 13;

// Evaluation table for number of possible 4-in-a-rows
const EVALTABLE: [i16; SIZE as usize] = [
	3, 4, 5,  7,  5,  4, 3,
	4, 6, 8,  10, 8,  6, 4,
	5, 8, 11, 13, 11, 8, 5,
	5, 8, 11, 13, 11, 8, 5,
	4, 6, 8,  10, 8,  6, 4,
	3, 4, 5,  7,  5,  4, 3
];

/// pair with (move, eval).
#[derive(Debug)]
pub struct EvalPair(usize, i16);

impl EvalPair {
    pub fn set_eval(&mut self, eval: i16) {
        self.1 = eval;
    }

    pub fn set_move(&mut self, mv: usize) {
        self.0 = mv;
    }

    pub fn get_eval(&self) -> i16 {
        self.1
    }

    pub fn get_move(&self) -> usize {
        self.0
    }
}


pub fn strategy(board: &mut Board) -> EvalPair {
    let color = if board.get_next_player() {-1} else {1};
    let alpha = -MAX_SCORE;
    let beta = MAX_SCORE;
    let pair: EvalPair;

    if color == 1 {
        pair = negamax(board, MIN_DEPTH, alpha, beta, color);
    }
    else {
        pair = negamax(board, MIN_DEPTH, -beta, -alpha, color);
    }

    pair
}

fn negamax(board: &mut Board, depth: u8, mut a: i16, b: i16, color: i16) -> EvalPair {
    let mut p = EvalPair(usize::MAX, i16::MIN);

    // if game over, get the evaluation and terminate
    if let Some(val) = game_over_eval(board, depth) {
        p.set_eval(val * color);
        return p;
    }

    // also do the same when max depth is reached
    else if depth == 0 {
        p.set_eval(color * eval(board));
        return p;
    }

    // obtains the valid moves
    for m in board.get_valid_moves() {
        board.add_unchecked(m);

        let eval_val = -negamax(board, depth - 1, -b, -a, -color).get_eval();

        board.undo();

        if eval_val > p.get_eval() {
            p.set_move(m);
            p.set_eval(eval_val);
        }

        a = i16::max(a, p.get_eval());
        if a >= b {
            break;
        }
    }

    p
}

/// returns None if not game over. Otherwise, will
/// return the evaluation of the board
pub fn game_over_eval(board: &Board, depth: u8) -> Option<i16> {
    let bitboards = board.get_bitboards();

    // if first player wins, return the positive
    if Board::is_win(bitboards[0]) {
        Some(MAX_SCORE + depth as i16)
    }

    // if second player wins, return negative
    else if Board::is_win(bitboards[1]) {
        Some(-(MAX_SCORE + depth as i16))
    }

    // if draw game
    else if board.is_filled() {
        Some(0)
    }

    // otherwise, the game is still ongoing.
    else {
        None
    }
}

/// returns the numerical evaluation of the given board position.
///
/// when this function is called, the board MUST NOT BE GAME OVER.
/// If it is GAME OVER, use the `game_over_eval` function instead.
pub fn eval(board: &Board) -> i16 {
    let mut r: usize;
    let mut c: usize;
    let mut total = 0;
    for (i, val) in EVALTABLE.into_iter().enumerate() {
        c = i % WIDTH;
        r = HEIGHT - i / WIDTH - 1;

        match board.get(r, c) {
            Some(false) => total += val,
            Some(true) => total -= val,
            _ => (),
        }
    }

    total
}
