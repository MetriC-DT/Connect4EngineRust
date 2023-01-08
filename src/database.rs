// Connect4EngineRust, a strong solver for the connect-4 board game.
// Copyright (C) 2023 Derick Tseng
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::moves::Moves;
use crate::strategy::Explorer;
use crate::board::{Board, SIZE};
use anyhow::Result;
use rand::seq::SliceRandom;
use rusqlite::Connection;

/// Each DBEntry contains information about the position. The `current player` represents the
/// player who is next to move. `opponent player` is the player who played the previous move.
/// The variables we store in each entry of the table are:
///
/// string of moves made to get to the current position.
/// move_history: String
///
/// number of moves played on the board.
/// moves_played: u32
///
/// Bitboard of the player who just made a move (the current player).
/// player_board: Position
///
/// Bitboard of the player who is next to move (the opponent player).
/// opp_board: Position
///
/// evaluation score. (Calculated with `MAX_SCORE - moves_played` at the final position,
/// assuming both sides played perfectly.)
/// eval: i8

const INSERT_STR: &str = "INSERT INTO positions (history, moves, player, opponent, eval) VALUES (?,?,?,?,?)";

/// Helper to generate a database of random legal positions for use in training the NNUE and
/// perhaps in generating a good openings database.
pub struct Database {
    connection: Connection,
}

impl Database {
    /// appends to a sqlite database in `filename`, if it exists, otherwise, creates a new
    /// connection to save to `filename`.
    pub fn new(filename: &str) -> Self {
        let conn = Connection::open(filename);
        match conn {
            Ok(connection) => {
                connection.execute_batch(
                    "BEGIN;
                    CREATE TABLE IF NOT EXISTS positions (
                        history TEXT,
                        moves INTEGER,
                        player INTEGER,
                        opponent INTEGER,
                        eval INTEGER
                    );
                    COMMIT;").unwrap();

                Self { connection }
            },
            Err(s) => panic!("{}", s),
        }
    }

    /// generates an entry for each position in the list.
    pub fn write_entries_from_list(&mut self, positions: &[String]) -> Result<()> {
        let mut explorer = Explorer::new();
        for position in positions {
            let board = Board::new_position(position)?;
            let eval = explorer.evaluate(&board);
            self.write_entry(&board, position, eval)?;
        }

        // commit all the changes to the connected database.
        self.connection.transaction()?.commit()?;
        Ok(())
    }

    /// writes a singular entry into the database. Still need to commit changes at the end.
    fn write_entry(&mut self, board: &Board, hist: &str, eval: i8) -> Result<()> {
        let mut stmt = self.connection.prepare_cached(INSERT_STR)?;
        let player = board.get_curr_player_pos();
        let opponent = board.get_opp_player_pos();
        let moves_played = board.moves_played();
        let entry = (hist, moves_played, player, opponent, eval);
        stmt.execute(entry)?;
        Ok(())
    }

    /// writes `num_entries` randomly generated entries to a file.
    /// Expects the `filename` to be a sqlite3 database.
    /// If `filename` exists as a sqlite3 database, then we append new entries into the table.
    /// Otherwise, creates a new file. There may be repeat entries in the database.
    /// `max_moves` and `min_moves` gives the inclusive bounds of the number of moves the entry can
    /// contain.
    pub fn write_entries_random(
        &mut self,
        num_entries: usize,
        max_moves: u8,
        min_moves: u8) -> Result<()> {

        let mut count = 0;
        let mut explorer = Explorer::new();

        while count < num_entries {
            // generates a random board position.
            let boards = Self::generate_random_board_positions(min_moves, max_moves);

            for (hist, board) in boards {
                // println!("Currently evaluating\n{}{}\n", board, hist);
                let eval = explorer.evaluate(&board);

                // insert into database.
                self.write_entry(&board, &hist, eval)?;

                // successfully added a new entry.
                count += 1;
                if count >= num_entries { break; }
            }
        }

        // commit all the changes to the connected database.
        self.connection.transaction()?.commit()?;
        Ok(())
    }

    /// generates a random legal board position. The corresponding string in the tuple of the
    /// vector is the move history.
    fn generate_random_board_positions(min_moves: u8, max_moves: u8) -> Vec<(String, Board)> {
        let (mut hist, mut board) = Self::generate_random_board(min_moves);
        let mut hist_board_pairs = Vec::new();

        // inserts the starting position into the board history.
        hist_board_pairs.push((hist.clone(), board));

        while !board.is_game_over() && board.moves_played() < max_moves.into() {
            let possible = board.possible_moves();
            let poss_moves: Vec<_> = Moves::new(possible).collect();
            let (mv_pos, col) = poss_moves.choose(&mut rand::thread_rng()).unwrap();
            board.play(*mv_pos);
            hist.push_str(&(col + 1).to_string());

            hist_board_pairs.push((hist.clone(), board));
        }

        hist_board_pairs
    }


    /// generates a random position, with the specified amount of moves played. The game may be
    /// completed at exactly `moves_played` but never before.
    fn generate_random_board(moves_played: u8) -> (String, Board) {
        let mut history = String::with_capacity(SIZE as usize + 1);
        let mut board = Board::new();

        while board.moves_played() < moves_played.into() {
            if board.is_game_over() { // reset if game over
                board = Board::new();
                history.clear();
            }

            // there should still be possible moves to play.
            let possible = board.possible_moves();
            let non_losing = board.non_losing_moves(possible);
            let next_moves: Vec<_> = if non_losing == 0 {
                Moves::new(possible).collect()
            } else {
                Moves::new(non_losing).collect()
            };

            let (mv_pos, mv) = next_moves.choose(&mut rand::thread_rng()).unwrap();
            board.play(*mv_pos);
            history.push_str(&(mv + 1).to_string());
        }

        (history, board)
    }
}
