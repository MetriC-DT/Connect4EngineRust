use crate::{board::{Board, SIZE}, transpositiontable::TranspositionTable, moves::MoveEvalPair};

pub const MAX_SCORE: i8 = 50;
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

pub fn strategy(board: &mut Board, table: &mut TranspositionTable) -> MoveEvalPair {
    let color = if board.get_next_player() {-1} else {1};
    let alpha = -MAX_SCORE;
    let beta = MAX_SCORE;

    if color == 1 {
        negamax(board, MIN_DEPTH, alpha, beta, color, table)
    }
    else {
        negamax(board, MIN_DEPTH, -beta, -alpha, color, table)
    }
}

fn negamax(board: &mut Board,
           depth: i8,
           mut a: i8,
           b: i8,
           color: i8,
           table: &mut TranspositionTable) -> MoveEvalPair {

    // if table contains the position, we return it.
    // usually, we would need to check depth. However,
    // because we want to evaluate the game to completion,
    // this should be fine.
    if let Some(pair) = table.get(board) {
        return *pair;
    }

    // if game over, get the evaluation and terminate
    else if let Some(val) = game_over_eval(board, depth) {
        return MoveEvalPair::new(u8::MAX, val * color);
    }

    let mut p = MoveEvalPair::new(u8::MAX, i8::MIN);

    // obtains the valid moves
    for m in board.get_valid_moves() {
        board.add_unchecked(m);
        let pair = negamax(board, depth - 1, -b, -a, -color, table);

        let eval_val = -pair.get_eval();
        table.insert(board, pair);

        board.undo();

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
pub fn game_over_eval(board: &Board, depth: i8) -> Option<i8> {
    // if first player wins, return the positive
    if board.is_first_player_win() {
        Some(MAX_SCORE + depth)
    }

    // if second player wins, return negative
    else if board.is_second_player_win() {
        Some(-(MAX_SCORE + depth))
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
