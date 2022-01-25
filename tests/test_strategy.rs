use connect4engine::{board::Board, strategy::{strategy, game_over_eval}, transpositiontable::TranspositionTable};

#[test]
fn test_endgame_1() {
    // piece 1 will win in this line.
    let o_win_string = "33333641111325545455152264404016060660202";
    let mut b = Board::new();

    let moves_until_end = 25;

    for (i, c) in o_win_string.chars().enumerate() {
        if i == o_win_string.len() - moves_until_end {
            break;
        }

        let col = c.to_digit(10).unwrap();
        b.add(col as u8).unwrap();
    }

    let mut turncount = 0;

    // runs until game is over.
	while game_over_eval(&b, 0).is_none() {
		let col = strategy(&mut b, &mut TranspositionTable::new()).get_move();
        b.add(col).unwrap();
		turncount += 1;
	}

    assert!(b.is_first_player_win());
    assert!(!b.is_second_player_win());
    assert_eq!(turncount, moves_until_end);
}
