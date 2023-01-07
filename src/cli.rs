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

use clap::{Parser, Subcommand, Args};

/// command line arguments for the program.
#[derive(Parser)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Test the evaluation of the engine.
    Test {
        /// file to run the test (choose from test_inputs folder)
        file: String
    },

    /// Play against the engine.
    Play {
        /// position to play against the engine.
        /// Leave empty to play from the starting position.
        position: Option<String>
    },

    /// Evaluate a given position.
    Eval { position: String },

    /// Create a database of positions.
    DB(DB)
}

#[derive(Args)]
pub struct DB {
    /// file to save the new database, or append if it already exists.
    pub file: String,

    /// number of elements to save to the database.
    #[arg(short, long, default_value_t=1000)]
    pub num: usize,

    /// minimum number of moves played required for each database entry.
    #[arg(long, default_value_t=0)]
    pub min: u8,

    /// maximum number of moves played required for each database entry.
    #[arg(long, default_value_t=42)]
    pub max: u8
}
