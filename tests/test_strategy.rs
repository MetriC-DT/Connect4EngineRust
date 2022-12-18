use connect4engine::{board::Board, strategy::Explorer};

#[test]
fn test_endgame_1() {
    // piece 1 will win in this line.
    let o_win_string = "44444752222436656566263375515127171771313";
    let (turncount, board) = run_game(o_win_string);

    assert!(board.is_first_player_win());
    assert!(!board.is_second_player_win());
    assert_eq!(0, turncount);
    assert_eq!(usize::from(board.moves_played()), o_win_string.len());
}

#[test]
fn test_endgame_2() {
    // player 2 forced win in 13 moves
    let line = "16357157437461355316457465722";
    let (turncount, board) = run_game(line);

    // player 2 forced win in 13 moves
    assert!(board.is_second_player_win());
    assert!(!board.is_first_player_win());
    assert_eq!(turncount, 13);
}

#[test]
fn test_endgame_3() {
    let line = "662222576343651642712157";
    let (turncount, board) = run_game(line);

    // player 1 forced win in 3 moves
    assert!(board.is_first_player_win());
    assert!(!board.is_second_player_win());
    assert_eq!(turncount, 3);
}

/// runs the game, returning (num_turns, resulting board)
fn run_game(line: &str) -> (usize, Board) {
    let board = Board::new_position(line);
    let mut explorer = Explorer::with_board(board);
    let mut turncount = 0;

    while !explorer.board.is_game_over() {
        let col = explorer.solve().get_move();
        explorer.board.add(col).unwrap();
        turncount += 1;
    }

    (turncount, explorer.board)
}
