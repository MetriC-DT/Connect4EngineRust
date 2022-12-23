use connect4engine::board::Board;

#[test]
fn test_moves_ordering() {
    let emptyboard = Board::new();
    let mvs = emptyboard.get_valid_moves();

    let ordering = [3, 2, 4, 1, 5, 0, 6];
    for (mv, col) in mvs.zip(ordering) {
        assert_eq!(mv, col);
    }
}
