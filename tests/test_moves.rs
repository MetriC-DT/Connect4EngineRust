use connect4engine::{board::Board, moves::MoveEvalPair};

#[test]
fn test_moves_ordering() {
    let emptyboard = Board::new();
    let mvs = emptyboard.get_valid_moves();

    let ordering = [3, 2, 4, 1, 5, 0, 6];
    for (mv, col) in mvs.zip(ordering) {
        assert_eq!(mv, col);
    }
}

#[test]
fn test_move_eval_pair_init() {
    let pair = MoveEvalPair::new(3, 4);
    assert_eq!(pair.get_move(), 3);
    assert_eq!(pair.get_eval(), 4);
}
