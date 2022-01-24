use connect4engine::moves::Moves;

#[test]
fn test_all_moves() {
    let moves = [3, 4, 2, 3, 5, 6, 1];
    let mut mvs = Moves::new();

    for mv in moves.iter().rev() {
        mvs.add_move(*mv);
    }

    assert_eq!(moves.len(), mvs.count());

    for (m1, m2) in mvs.zip(moves) {
        assert_eq!(m1, m2);
    }
}

#[test]
fn test_partial() {
    let moves = [6, 1, 3];
    let mut mvs = Moves::new();

    for mv in moves.iter().rev() {
        mvs.add_move(*mv);
    }

    assert_eq!(moves.len(), mvs.count());

    for (m1, m2) in mvs.zip(moves) {
        assert_eq!(m1, m2);
    }
}
