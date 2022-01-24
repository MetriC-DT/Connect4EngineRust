/// compact representation for move generation.
///
/// There could be a max of 7 possible moves (0 - 6). Therefore,
/// if each move could be represented as 3 bits, it could be completed in
/// 28 bits.
///
/// The first 4 bits represent the number of items contained in `Moves` struct.
/// The last 7 elements (4 bits each) represent the moves (the possible columns).
#[derive(Clone, Copy)]
pub struct Moves {
    moves: u32
}

impl Moves {
    pub fn new() -> Self {
        Self { moves: 0 }
    }

    /// adds mv to the list of moves.
    ///
    /// Undefined behaviour if `mv` cannot be represented by 4 bits. This 
    /// should not be a problem internally due to the columns only being from 0-6.
    ///
    /// Also undefined behaviour if `mv` adds more than 7 moves. Also not a problem
    /// internally since there can only be 7 max moves.
    pub fn add_move(&mut self, mv: usize) {
        let mask = 0b1111;
        let count = (mask & self.moves) + 1;

        let mask_inv = !mask;
        self.moves &= mask_inv;
        self.moves |= count;

        // starts inserting from the back. This makes it easier for the
        // iterator to read the numbers in the correct order of insertion.
        let movemask = (mv as u32) << (count * 4);
        self.moves |= movemask;
    }
}

/// iterator over the elements inside Moves.
///
/// NOTE THAT THIS RETURNS THE ELEMENTS IN REVERSE ORDER FROM THE ORDER OF ADDITION (FILO)
impl Iterator for Moves {
    type Item=usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut mask = 0b1111;
        let count = mask & self.moves;

        if count > 0 {
            // stores (count - 1) back in to increment the iterator.
            let mask_inv = !mask;
            self.moves &= mask_inv;
            self.moves |= count - 1;

            // obtains the next move
            mask <<= count * 4;
            let res = (mask & self.moves) >> (count * 4);

            Some(res as usize)
        }
        else {
            None
        }
    }
}
