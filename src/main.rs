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

use clap::Parser;
use connect4engine::board::SIZE;
use connect4engine::cli::{Cli, Commands};
use connect4engine::database::Database;
use connect4engine::moves::EMPTY_MOVE;
use connect4engine::{strategy::Explorer, board::Board};
use std::fs;
use std::time::Instant;
use std::io::{self, BufReader, BufRead, Write};
use anyhow::Result;

/// main function
fn main() -> Result<()> {
    let cli = Cli::parse();

    // if no command inputted, run the stdin.
    if cli.command.is_none() {
        eval_from_stdin()?;
        return Ok(())
    }

    // command was inputted. Need to parse.
    match &cli.command.unwrap() {
        Commands::Test { file } => test_files(file)?,
        Commands::Eval { position } => eval_position(position)?,
        Commands::Play { position } => play_position(position.as_deref())?,
        Commands::DB(db) => create_database(&db.file, db.max, db.min, db.num, db.stdin)?,
    };

    Ok(())
}

/// creates a sqlite3 database of positions at the specified location.
fn create_database(filename: &str, max: u8, min: u8, num: usize, stdin: bool) -> Result<()> {
    let mut db = Database::new(filename);

    if stdin { // positions from stdin.
        let mut positions = Vec::new();

        loop {
            let mut buf = String::with_capacity(SIZE as usize + 1);
            let r = io::stdin().read_line(&mut buf)?;
            if r == 0 { break; }
            let strpos = String::from(buf.trim());
            positions.push(strpos);
        }

        // put positions in the database.
        db.write_entries_from_list(positions.as_slice())?;
    }
    else { // generate random positions
        db.write_entries(num, max, min)?;
    }
    Ok(())
}

/// prints the evaluation and optimal move for a given position.
fn eval_position(pos: &str) -> Result<()> {
    let board = Board::new_position(pos)?;
    let mut explorer = Explorer::new();
    let (mv, eval) = explorer.solve(&board);

    print_eval(mv, eval);
    Ok(())
}


/// reads positions from stdin and outputs the evaluation and best move into stdout.
fn eval_from_stdin() -> Result<()> {
    let mut buf = String::new();
    let mut explorer = Explorer::new();

    loop {
        buf.clear();
        let r = io::stdin().read_line(&mut buf)?;

        // EOF was reached.
        if r == 0 { break; }

        // we want to write the corresponding (mv, eval) pair.
        let b = Board::new_position(buf.trim());
        if let Err(s) = b {
            // if cannot enter new position, print the error message.
            println!("{}", s);
            continue;
        }

        // new position has been inputted. We can solve.
        let board = b.unwrap();
        let (mv, eval) = explorer.solve(&board);
        print_eval(mv, eval);

        // flush output immediately.
        io::stdout().flush()?;
    }

    Ok(())
}

/// Prints the evaluation and move, taking care of game over scenarios.
fn print_eval(mv: u8, eval: i8) {
    if mv == EMPTY_MOVE {
        // is already game over.
        println!("GameOver (Eval: {})", eval);
    }
    else {
        // we want to output the columns in [1-7].
        println!("Best Move: {} (Eval: {})", mv + 1, eval);
    }
}


/// plays the game from the given position.
fn play_position(position: Option<&str>) -> Result<()> {
    let mut board;
    let mut pos_str;

    if let Some(s) = position {
        pos_str = String::from(s);
        board = Board::new_position(s)?;
    }
    else {
        pos_str = String::new();
        board = Board::new();
    }

    let mut explorer = Explorer::new();

    loop {
        println!("{}\n{}\n--------------------------------", board, pos_str);

        println!("Waiting for engine to generate move...");
        let (mv, eval) = explorer.solve(&board);

        if mv == EMPTY_MOVE { // used when the given game is already over.
            break;
        }

        let result = board.add(mv);
        if let Err(s) = result {
            panic!("Engine corrupted. Tried to put in column {}. Aborting. {:?}", mv + 1, s);
        }
        else {
            let mv_played = mv + 1;
            println!("Engine played {} (Eval {})", mv_played, eval);
            pos_str.push_str(&format!("{}", mv_played));
        }

        if board.has_winner() || board.is_filled() {
            break;
        }

        println!("{}\n{}\n--------------------------------", board, pos_str);
        // get user input.
        loop {
            let mut buf = String::new();
            print!("Enter column [1-7] > ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut buf)?;

            buf = buf.trim().to_string();
            if buf.len() != 1 {
                println!("Can only take 1 character.");
                continue;
            }

            let player_mv = buf.chars().next();

            if player_mv.is_none() {
                println!("Not a valid move.");
                continue;
            }

            let player_mv = player_mv.unwrap().to_digit(10);
            if player_mv.is_none() {
                println!("Input does not appear to be a number.");
                continue;
            }

            let player_mv: Result<u8, _> = player_mv.unwrap().try_into();
            if let Err(s) = player_mv {
                println!("Not a valid move. {}", s);
                continue;
            }

            let player_mv = player_mv.unwrap().checked_sub(1);
            if player_mv.is_none() {
                println!("Not a valid column");
                continue;
            }

            if let Err(s) = board.add(player_mv.unwrap()) {
                println!("{}", s);
                continue;
            }

            // user input successful.
            pos_str.push_str(&format!("{}", player_mv.unwrap() + 1));
            break;
        }

        if board.has_winner() || board.is_filled() {
            break;
        }
    }

    println!("{}\n{}\n--------------------------------", board, pos_str);
    println!("Game Over!");
    Ok(())
}

/// runs all of the tests from the given test file.
/// The format of the test file is:
/// [position] [evaluation]
fn test_files(filename: &str) -> Result<()> {
    let file = fs::File::open(filename)?;
    let reader = BufReader::new(file);
    let mut explorer = Explorer::new();

    let mut totaltime = 0;
    let mut count = 0;

    for line in reader.lines() {
        let linestr = line?;
        let linestr = linestr.split(' ').next().unwrap();
        count += 1;
        let next_board = Board::new_position(linestr)?;

        // time the solve
        let start_time = Instant::now();
        let eval = explorer.evaluate(&next_board);
        let delta = start_time.elapsed().as_micros();
        totaltime += delta;

        println!("{}\t{}", &linestr.split(' ').next().unwrap(), eval);
        io::stdout().flush()?
    }

    let nodecount = explorer.get_nodes_explored();

    println!();
    println!("time elapsed:        {}us", totaltime);
    println!("positions evaluated: {}", nodecount);
    println!("speed:               {} Kpos/s", nodecount as f32 / totaltime as f32 * 1000.0);
    println!("Avg time:            {} us", totaltime as f32 / count as f32);
    println!("Avg nodes:           {}", nodecount as f32 / count as f32);

    Ok(())
}
