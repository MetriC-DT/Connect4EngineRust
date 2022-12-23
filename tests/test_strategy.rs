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

#[test]
fn test_endgame_4() {
    let line = "2252576253462244111563365343671351441";
    let (turncount, board) = run_game(line);

    assert!(board.is_first_player_win());
    assert!(!board.is_second_player_win());
    assert_eq!(turncount, 4);
}

#[test]
fn test_endgame_5() {
    let line = "52677675164321472411331752454";
    let (turncount, board) = run_game(line);
    assert!(board.is_filled());
    assert!(!board.is_first_player_win());
    assert!(!board.is_second_player_win());
    assert_eq!(turncount, board.moves_played() as usize - line.len())
}

#[test]
fn test_endgame_6() {
    let line = "65214673556155731566316327373221417";
    let (turncount, board) = run_game(line);
    assert!(board.is_first_player_win());
    assert!(!board.is_second_player_win());
    assert!(!board.is_filled());
    assert_eq!(turncount, board.moves_played() as usize - line.len())
}

/// runs the game, returning (num_turns, resulting board)
fn run_game(line: &str) -> (usize, Board) {
    let board = Board::new_position(line);
    let mut explorer = Explorer::with_board(board);
    let mut turncount = 0;
    println!("{}", board);

    while !explorer.get_board().is_game_over() {
        let (col, val) = explorer.solve().get_pair();
        let mut new_pos_board = explorer.get_board().clone();
        assert!(new_pos_board.add(col).is_ok());

        explorer.change_board(&new_pos_board);
        println!("Move {} Eval {}\n{}", col + 1, val, new_pos_board);
        turncount += 1;
    }

    (turncount, *explorer.get_board())
}
