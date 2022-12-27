use connect4engine::{board::{HEIGHT, Board, WIDTH}, moves::DEFAULT_ORDER};

#[test]
fn test_board_add() {
    let mut b = Board::new();
    let col: u8 = 3;
    for i in 0..HEIGHT {
        assert_eq!(b.get_height(col), i);
        assert!(b.add(col).is_ok());
    }

    // cannot add when height max is reached.
    assert!(b.add(col).is_err());
    assert_eq!(b.get_height(col), HEIGHT);
}

fn test_winning_line(line: &str) {
    let mut b = Board::new();
    // play up until the last move, but not the last one.
    for c in line[..line.len() - 1].chars() {
        let col: u8 = c.to_digit(10).unwrap() as u8 - 1;
        assert!(b.add(col).is_ok());
        assert!(!b.is_filled());
        assert!(!b.has_winner());
    }

    let col: char = line.as_bytes()[line.len() - 1] as char;
    let col: u8 = col.to_digit(10).unwrap() as u8 - 1;
    assert!(b.add(col).is_ok());
    assert!(b.has_winner());
}

#[test]
fn test_win_vertical() {
    let line = "3232323";
    test_winning_line(line);
}

#[test]
fn test_win_horizontal() {
    let line = "1122334";
    test_winning_line(line);
}

#[test]
fn test_valid_moves() {
    let mut b = Board::new();
    let moves = b.get_valid_moves();
    assert_eq!(moves.clone().count(), WIDTH as usize);

    // has all columns available
    for (mv, col) in moves.clone().zip(DEFAULT_ORDER) {
        assert_eq!(Board::pos_to_col(mv), col)
    }

    // fills the 3rd and 6th columns
    for _ in 0..HEIGHT {
        b.add(3).unwrap();
        b.add(6).unwrap();
    }

    let moves = b.get_valid_moves();
    assert_eq!(moves.count(), (WIDTH - 2) as usize);

    // has these columns
    let has_moves = [0, 1, 2, 4, 5];
    for mv in b.get_valid_moves() {
        let col = Board::pos_to_col(mv);
        assert!(has_moves.contains(&col));
    }

    // does not have these columns
    let not_has_moves = [3, 6];
    for mv in b.get_valid_moves() {
        let col = Board::pos_to_col(mv);
        assert!(!not_has_moves.contains(&col));
    }
}

#[test]
fn test_filled() {
    let mut b = Board::new();
    for _ in 0..HEIGHT {
        for col in 0..WIDTH {
            assert!(b.add(col).is_ok());
        }
    }

    assert!(b.is_filled());
}

#[test]
fn test_not_filled() {
    let mut b = Board::new();
    for col in 0..WIDTH {
        b.add(col).unwrap();
    }

    assert!(!b.is_filled());
}

#[test]
fn test_unique_position_key() {
    let mut b = Board::new();
    let moves = "333336411113255454551522644040160606602022";
    let mut seen_keys = Vec::new();

    // simulates a game to check if keys collide
    for col in moves.chars() {
        let unique_position_key = b.get_unique_position_key();
        assert!(!seen_keys.contains(&unique_position_key));
        seen_keys.push(unique_position_key);

        let col = col.to_digit(10).unwrap();
        b.add(col as u8).unwrap();
    }

    // checks last one
    let unique_position_key = b.get_unique_position_key();
    assert!(!seen_keys.contains(&unique_position_key));
}
