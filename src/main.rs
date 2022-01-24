use connect4engine::{board::Board, strategy};

fn main() {
    println!("Welcome to Connect 4");

    let mut b = Board::new();
    let moves = b.get_valid_moves();

    for mv in moves {
        println!("{}", mv);
    }

    b.add(3).unwrap();
    b.add(3).unwrap();

    println!("{:?}", strategy::strategy(&mut b));
}
