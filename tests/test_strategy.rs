use connect4engine::{board::Board, strategy::Explorer};

#[test]
fn test_endgame_1() {
    // piece 1 will win in this line.
    let o_win_string = "33333641111325545455152264404016060660202";

    let mut b = Board::new();
    let moves_until_end = 20;

    for (i, c) in o_win_string.chars().enumerate() {
        if i == o_win_string.len() - moves_until_end {
            break;
        }

        let col = c.to_digit(10).unwrap();
        b.add(col as u8).unwrap();
    }

    let mut explorer = Explorer::with_board(b);
    let mut turncount = 0;

    // runs until game is over.
	while !explorer.board.is_game_over() {
		let col = explorer.strategy().get_move();
        explorer.board.add(col).unwrap();
		turncount += 1;
	}

    assert!(explorer.board.is_first_player_win());
    assert!(!explorer.board.is_second_player_win());
    assert_eq!(turncount, moves_until_end);

    println!("{}", explorer.get_nodes_explored())
}

#[test]
fn test_endgame_2() {
    // player 2 forced win in 13 moves
    let line = "16357157437461355316457465722";
    let board = Board::new_position(line);
    let mut explorer = Explorer::with_board(board);
    let mut turncount = 0;

    while !explorer.board.is_game_over() {
        let col = explorer.strategy().get_move();
        explorer.board.add(col).unwrap();
        turncount += 1;
    }

    // player 2 forced win in 13 moves
    assert!(explorer.board.is_second_player_win());
    assert!(!explorer.board.is_first_player_win());
    assert_eq!(turncount, 13);
}
