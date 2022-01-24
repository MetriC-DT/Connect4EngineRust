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
    assert!(b.is_first_player_win());
    assert!(!b.is_second_player_win());
}

#[test]
fn test_win_horizontal() {
    let mut b = Board::new();
    b.add(0).unwrap();
    b.add(0).unwrap();
    b.add(1).unwrap();
    b.add(1).unwrap();
    b.add(2).unwrap();
    b.add(2).unwrap();
    b.add(3).unwrap();

    assert!(b.is_first_player_win());
    assert!(!b.is_second_player_win());
}

#[test]
fn test_valid_moves() {
    let mut b = Board::new();
    let moves = b.get_valid_moves();
    assert_eq!(moves.count(), WIDTH);
    // has these values
    assert!(moves.clone().find(|&x| x == 0).is_some());
    assert!(moves.clone().find(|&x| x == 1).is_some());
    assert!(moves.clone().find(|&x| x == 2).is_some());
    assert!(moves.clone().find(|&x| x == 3).is_some());
    assert!(moves.clone().find(|&x| x == 4).is_some());
    assert!(moves.clone().find(|&x| x == 5).is_some());
    assert!(moves.clone().find(|&x| x == 6).is_some());

    // fills the 3rd and 6th columns
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
    assert_eq!(moves.count(), (WIDTH - 2) as usize);

    // cannot put in these columns
    assert!(moves.clone().find(|&x| x == 3).is_none());
    assert!(moves.clone().find(|&x| x == 6).is_none());

    // has these columns
    assert!(moves.clone().find(|&x| x == 0).is_some());
    assert!(moves.clone().find(|&x| x == 1).is_some());
    assert!(moves.clone().find(|&x| x == 2).is_some());
    assert!(moves.clone().find(|&x| x == 4).is_some());
    assert!(moves.clone().find(|&x| x == 5).is_some());
}
