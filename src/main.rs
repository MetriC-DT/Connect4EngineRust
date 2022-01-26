use connect4engine::{strategy::Explorer, board::Board};
use std::{fs, io::{BufRead, self, BufReader}, time::Instant};

fn main() -> io::Result<()> {
    let file = fs::File::open("Test_L3_R1.txt")?;
    let reader = BufReader::new(file);
    let mut explorer = Explorer::new();

    let mut totaltime = 0;

    for line in reader.lines() {
        explorer.change_board(Board::new_position(&line?));
        let start_time = Instant::now();
        let evaluation = explorer.strategy();
        let delta = start_time.elapsed().as_micros();
        totaltime += delta;

        println!("{}\t{}\t{}us", evaluation.get_eval(), explorer.get_nodes_explored(), delta);
    }

    let nodecount = explorer.get_nodes_explored();

    println!("\ntime elapsed: {}\npositions evaluated: {}\nspeed: {}",
             totaltime,
             nodecount,
             nodecount as f32 / totaltime as f32 * 1000.0);

    Ok(())
}
