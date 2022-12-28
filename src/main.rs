use connect4engine::{strategy::Explorer, board::Board};
use std::fs;
use std::time::Instant;
use std::io::{self, BufReader, BufRead, Write};
use clap::Parser;
use anyhow::Result;

/// command line arguments to use.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {

    /// name of the test file to run (format inside test_inputs)
    #[arg(short, long)]
    test_file: Option<String>,

    /// plays the given position (empty string for new position)
    #[arg(short, long)]
    play: Option<String>,
}

/// main function
fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(filename) = args.test_file {
        test_files(&filename)?;
        Ok(())
    }

    // Plays from the given position, if it exists. default creates a new board.
    else if let Some(position) = args.play {
        play_position(&position)?;
        Ok(())
    }

    else {
        // TODO - read from stdin.
        todo!("TODO - Reading from stdin incomplete.")
    }
}


/// plays the game from the given position.
fn play_position(position: &str) -> Result<()> {
    let mut board = Board::new_position(position);

    let mut explorer = Explorer::new();

    loop {
        println!("{}", board);

        explorer.change_board(&board);
        println!("Waiting for engine to generate move...");
        let (mv, _eval) = explorer.solve();
        let result = board.add(mv);
        if let Err(s) = result {
            panic!("Engine corrupted. Aborting.\n{:?}", s);
        }
        else {
            println!("Engine played {}", mv + 1);
        }

        println!("{}", board);
        if board.has_winner() || board.is_filled() {
            break;
        }

        // get user input.
        loop {
            let mut buf = String::new();
            print!("Enter column [1-7] > ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut buf)?;
            let player_mv = buf.chars().next();

            if let None = player_mv {
                println!("Not a valid move.");
                continue;
            }

            let player_mv = player_mv.unwrap().to_digit(10);
            if let None = player_mv {
                println!("Input does not appear to be a number.");
                continue;
            }

            let player_mv: Result<u8, _> = player_mv.unwrap().try_into();
            if let Err(s) = player_mv {
                println!("Not a valid move. {}", s);
                continue;
            }

            let player_mv = player_mv.unwrap().checked_sub(1);
            if let None = player_mv {
                println!("Not a valid column");
                continue;
            }

            if let Err(s) = board.add(player_mv.unwrap()) {
                println!("{}", s);
                continue;
            }
            break;
        }

        if board.has_winner() || board.is_filled() {
            break;
        }
    }

    println!("Game Over!");
    Ok(())
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

        // time the solve
        let start_time = Instant::now();
        let (_mv, eval) = explorer.solve();
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
