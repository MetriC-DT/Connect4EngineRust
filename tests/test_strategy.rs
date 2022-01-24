use connect4engine::{board::Board, strategy::{MIN_DEPTH, strategy, game_over_eval}};

#[test]
fn test_finish_1() {
    // piece 1 will win in this line.
    let o_win_string = "33333641111325545455152264404016060660202";
    let mut b = Board::new();

    for (i, c) in o_win_string.chars().enumerate() {
        if i == o_win_string.len() - MIN_DEPTH as usize {
            break;
        }

        let col = c.to_digit(10).unwrap();
        b.add(col as usize).unwrap();
    }

    let mut turncount = 0;
	while game_over_eval(&b, 0).is_none() {
		let col = strategy(&mut b).get_move();
        b.add(col).unwrap();
		turncount += 1;
	}

    let boards = b.get_bitboards();
    assert!(Board::is_win(boards[0]));
    assert!(!Board::is_win(boards[1]));
    assert_eq!(turncount, MIN_DEPTH);
}
