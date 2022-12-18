use connect4engine::{strategy::Explorer, board::Board};
use std::{fs, io::{BufRead, self, BufReader}, time::Instant, env};
use std::io::{stdout, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    test_files(args[1].as_str())
}

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
        let evaluation = explorer.solve();
        let delta = start_time.elapsed().as_micros();
        totaltime += delta;

        println!("{}\t{}\t{}us\t{}",
                 &linestr.split(' ').next().unwrap(),
                 evaluation.get_eval(),
                 explorer.get_nodes_explored() - prev_nodecount,
                 delta);

        stdout().flush()?
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
