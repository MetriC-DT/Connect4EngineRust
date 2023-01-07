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
use crate::board::{Position, Board};
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

                return Self { connection }
            },
            Err(s) => panic!("{}", s),
        }
    }

    /// writes `num_entries` randomly generated entries to a file.
    /// Expects the `filename` to be a sqlite3 database.
    /// The number of batches that will be written can be calculated with
    /// `ceil(num_entries / batch_size)`. If `filename` exists as a sqlite3 database, then we
    /// append new entries into the table. Otherwise, creates a new file. There may be repeat
    /// entries in the database.
    /// `max_moves` and `min_moves` gives the inclusive bounds of the number of moves the entry can
    /// contain.
    pub fn write_entries(
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
                let eval = explorer.evaluate(&board);
                // println!("{}{}\n", board, hist);

                // insert into database.
                let mut stmt = self.connection.prepare_cached(INSERT_STR)?;
                let player = board.get_curr_player_pos();
                let opponent = board.get_opp_player_pos();
                let moves_played = board.moves_played();
                let entry = (hist, moves_played, player, opponent, eval);
                stmt.execute(entry)?;

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
        let mut boards = Vec::new();

        let mut board = Board::new();
        let mut hist = String::with_capacity(max_moves.into());

        loop {
            let moves_played = board.moves_played() as u8;
            let next_moves: Vec<(Position, u8)>;

            if moves_played < min_moves {
                // want to play moves that don't lose immediately.
                let possible = board.possible_moves();
                let non_losing = board.non_losing_moves(possible);
                next_moves = Moves::new(non_losing).collect();
            }

            else if min_moves <= moves_played && moves_played <= max_moves {
                // saves the board's current position.
                boards.push((hist.clone(), board.clone()));

                // board is not game over, so we can keep playing moves.
                let possible = board.possible_moves();
                next_moves = Moves::new(possible).collect();
            }

            else {
                // we have exceeded the maximum number of moves.
                break;
            }

            if next_moves.len() == 0 || board.is_game_over() { // no possible next moves.
                if moves_played < min_moves {
                    // reset if we have not found a valid position with at least min_moves.
                    board = Board::new();
                    hist.clear();
                    continue;
                }
                else {
                    break;
                }
            }
            else {
                // there is at least 1 possible next move.
                // chooses a random move from possible_moves. There should at least be one possible
                // move since the game is not over.
                let (mv_pos, mv) = next_moves.choose(&mut rand::thread_rng()).unwrap();
                board.play(*mv_pos);
                hist.push_str(&(mv + 1).to_string());
            }
        }

        boards
    }
}
