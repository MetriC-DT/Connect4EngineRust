use connect4engine::{board::Board, strategy, transpositiontable::TranspositionTable};

fn main() {
    println!("Welcome to Connect 4");

    let mut b = Board::new();
    b.add(3).unwrap();
    b.add(3).unwrap();

    let mut table = TranspositionTable::new();
    println!("{:?}", strategy::strategy(&mut b, &mut table));
}
