use crate::board::{WIDTH, Board};

const DEFAULT_ORDER: [u8; WIDTH as usize] = [3, 2, 4, 1, 5, 0, 6];

/// representation for the allowed moves on a player's turn.
#[derive(Clone, Copy)]
pub struct Moves {
    board: u64,
    pointer: usize,
}

impl Moves {
    pub fn new(bits: u64) -> Self {
        Self { board: bits, pointer: 0 }
    }
}

/// iterator over the possible moves that can be played.
impl Iterator for Moves {
    type Item=u8;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.pointer;
        for &col in &DEFAULT_ORDER[i..] {
            self.pointer += 1;
            if !Board::col_is_occupied(self.board, col) {
                return Some(col);
            }
        }

        None
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MoveEvalPair(u8, i8);

impl MoveEvalPair {
    pub fn new(mv: u8, score: i8) -> Self {
        Self(mv, score)
    }
    pub fn set_eval(&mut self, eval: i8) {
        self.1 = eval;
    }

    pub fn set_move(&mut self, mv: u8) {
        self.0 = mv;
    }

    pub fn get_eval(&self) -> i8 {
        self.1
    }

    pub fn get_move(&self) -> u8 {
        self.0
    }
}
