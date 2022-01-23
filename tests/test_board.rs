use connect4engine::board::{HEIGHT, Board, WIDTH};

#[test]
fn test_board_add() {
    let mut b = Board::new();
    let col = 3;
    for i in 0..HEIGHT {
        assert_eq!(b.get_height(col), i);
        assert!(b.add(col).is_ok());
    }

    // cannot add when height max is reached.
    assert!(b.add(col).is_err());
    assert_eq!(b.get_height(col), HEIGHT);
}

#[test]
fn test_undo_1() {
    let mut b = Board::new();
    let col = 2;

    assert!(b.add(col).is_ok());
    assert_eq!(b.get_height(col), 1);

    b.undo();
    assert_eq!(b.get_height(col), 0);
}

#[test]
fn test_undo_2() {
    let mut b = Board::new();
    let col = 3;
    for i in 0..HEIGHT {
        assert_eq!(b.get_height(col), i);
        assert!(b.add(col).is_ok());
    }

    for i in 0..HEIGHT {
        assert_eq!(b.get_height(col), HEIGHT - i);
        b.undo();
    }

    assert_eq!(b.get_height(col), 0);
}

#[test]
fn test_win_vertical() {
    let mut b = Board::new();
    b.add(3).unwrap();
    b.add(2).unwrap();
    b.add(3).unwrap();
    b.add(2).unwrap();
    b.add(3).unwrap();
    b.add(2).unwrap();
    b.add(3).unwrap();

    let boards = b.get_bitboards();

    assert!(Board::is_win(boards[0]));
    assert!(!Board::is_win(boards[1]));
}

#[test]
fn test_valid_moves() {
    let mut b = Board::new();
    let moves = b.get_valid_moves();
    assert_eq!(moves.len(), WIDTH as usize);

    b.add(3).unwrap();
    b.add(3).unwrap();
    b.add(3).unwrap();
    b.add(3).unwrap();
    b.add(3).unwrap();
    b.add(3).unwrap();
    b.add(6).unwrap();
    b.add(6).unwrap();
    b.add(6).unwrap();
    b.add(6).unwrap();
    b.add(6).unwrap();
    b.add(6).unwrap();

    let moves = b.get_valid_moves();
    assert!(!moves.contains(&3));
    assert!(!moves.contains(&6));
}
