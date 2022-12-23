use connect4engine::moves::EMPTY_MOVE;
use connect4engine::{strategy::Explorer, board::Board};
use std::fs;
use std::time::Instant;
use std::io::{self, BufReader, BufRead, Write};
use clap::Parser;


/// command line arguments to use.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {

    /// name of the test file to run (format inside test_inputs)
    #[arg(short, long)]
    test_file: Option<String>,

    /// position to analyze.
    #[arg(short, long)]
    position: Option<String>,
}

/// main function
fn main() -> Result<(), String> {
    let args = Args::parse();

    if let Some(filename) = args.test_file {
        match test_files(&filename) {
            Ok(()) => Ok(()),
            Err(s) => Err(s.to_string()),
        }
    }

    // explores the position, returning the evaluation and move.
    else if let Some(position) = args.position {
        eval_position(&position);
        Ok(())
    }

    else {
        // TODO - run repl loop.
        Err("TODO - Repl to be implemented.".to_string())
    }
}


/// evaluates the position. Prints out string.
fn eval_position(position: &str) {
    let board = Board::new_position(position);
    println!("{}", board);

    let mut explorer = Explorer::with_board(board);
    let (mv, eval) = explorer.solve();

    if mv == EMPTY_MOVE {
        let eval = eval * board.get_prev_player_signed();
        println!("The game is already over. (Eval: {})", eval);
    }
    else {
        let eval = eval * board.get_current_player_signed();
        let readable_mv = mv + 1; // mv used internally is 0-indexed.
        println!("Best Move: {} (Eval: {})", readable_mv, eval);
    }
}

/// runs all of the tests from the given test file.
/// The format of the test file is:
/// [position] [evaluation]
fn test_files(filename: &str) -> io::Result<()> {
    let file = fs::File::open(filename)?;
    let reader = BufReader::new(file);
    let mut explorer = Explorer::new();

    let mut totaltime = 0;
    let mut count = 0;

    for line in reader.lines() {
        let linestr = line?;
        count += 1;
        explorer.change_board(&Board::new_position(&linestr));
        let prev_nodecount = explorer.get_nodes_explored();

        // time the solve
        let start_time = Instant::now();
        let (_, eval) = explorer.solve();
        let delta = start_time.elapsed().as_micros();
        totaltime += delta;

        println!("{}\t{}\t{}us\t{}",
                 &linestr.split(' ').next().unwrap(),
                 eval,
                 explorer.get_nodes_explored() - prev_nodecount,
                 delta);

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
