use connect4engine::{board::Board, transpositiontable::TranspositionTable};

#[test]
fn test_insert_get() {
    let board = Board::new_position("44444752222436656566263375515127171771313");
    let fake_eval = 3;

    let mut table = TranspositionTable::new();
    assert!(table.get(&board).is_none());

    table.insert(&board, fake_eval);
    assert!(table.get(&board).is_some());
    assert_eq!(table.get(&board).unwrap(), fake_eval);
}
