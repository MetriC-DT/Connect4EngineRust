use connect4engine::{board::Board, transpositiontable::TranspositionTable};

#[test]
fn test_insert_get() {
    let board = Board::new_position("44444752222436656566263375515127171771313");
    let eval_hi = 5;

    let mut table = TranspositionTable::new();
    assert!(table.get(&board).is_none());

    table.insert(&board, eval_hi);
    assert_eq!(table.get(&board).unwrap(), eval_hi);
}

#[test]
fn test_insert_get_negative() {
    let board = Board::new_position("44444752222436656566263375515127171771313");
    let eval_hi = -1;

    let mut table = TranspositionTable::new();
    assert!(table.get(&board).is_none());

    table.insert(&board, eval_hi);
    assert_eq!(table.get(&board).unwrap(), eval_hi);
}

#[test]
fn test_evict() {
    let board = Board::new_position("44444752222436656566263375515127171771313");
    let eval_hi = 10;

    let mut table = TranspositionTable::new();
    assert!(table.get(&board).is_none());

    table.insert(&board, eval_hi);
    assert_eq!(table.get(&board).unwrap(), eval_hi);

    table.insert(&board, eval_hi + 5);
    assert_eq!(table.get(&board).unwrap(), eval_hi + 5);
}
