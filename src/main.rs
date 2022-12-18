use connect4engine::{strategy::Explorer, board::Board};
use std::{fs, io::{BufRead, self, BufReader}, time::Instant, env};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(args[1].as_str())?;
    let reader = BufReader::new(file);
    let mut explorer = Explorer::new();

    let mut totaltime = 0;
    let mut count = 0;

    for line in reader.lines() {
        count += 1;
        explorer.change_board(&Board::new_position(&line?));
        let start_time = Instant::now();
        let evaluation = explorer.solve();
        let delta = start_time.elapsed().as_micros();
        totaltime += delta;

        println!("{}\t{}\t{}us", explorer.get_nodes_explored(), evaluation.unwrap().get_eval(), delta);
    }

    let nodecount = explorer.get_nodes_explored();

    println!("\ntime elapsed: {}us\npositions evaluated: {}\nspeed: {} Kpos/s\nAvg time: {}us\nAvg Nodes: {}",
             totaltime,
             nodecount,
             nodecount as f32 / totaltime as f32 * 1000.0,
             totaltime as f32 / count as f32,
             nodecount as f32 / count as f32);

    Ok(())
}
