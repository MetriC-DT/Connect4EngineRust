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
    file: String,

    /// number of elements to save to the database. 1000 by default.
    num: Option<usize>,

    /// minimum number of moves played required for each database entry.
    min_moves: Option<u8>,

    /// maximum number of moves played required for each database entry.
    max_moves: Option<u8>
}
