use connect4engine::board::Board;
use connect4engine::transpositiontable::TranspositionTable;
use connect4engine::transpositiontable::FLAG_UPPER;
use connect4engine::transpositiontable::FLAG_LOWER;

#[test]
fn test_insert_get() {
    let board = Board::new_position("44444752222436656566263375515127171771313").unwrap();
    let eval_hi = 5;

    let mut table = TranspositionTable::new();
    assert!(table.get_entry(&board).is_none());

    table.insert(&board, eval_hi, FLAG_UPPER);
    let entry = table.get_entry(&board).unwrap();
    assert_eq!(entry.get_eval(), eval_hi);
    assert_eq!(entry.get_flag(), FLAG_UPPER);
}

#[test]
fn test_insert_get_negative() {
    let board = Board::new_position("44444752222436656566263375515127171771313").unwrap();
    let eval_hi = -1;

    let mut table = TranspositionTable::new();
    assert!(table.get_entry(&board).is_none());

    table.insert(&board, eval_hi, FLAG_UPPER);
    let entry = table.get_entry(&board).unwrap();
    assert_eq!(entry.get_eval(), eval_hi);
    assert_eq!(entry.get_flag(), FLAG_UPPER);
}

#[test]
fn test_evict() {
    let board = Board::new_position("44444752222436656566263375515127171771313").unwrap();
    let eval_hi = 10;

    let mut table = TranspositionTable::new();
    assert!(table.get_entry(&board).is_none());

    table.insert(&board, eval_hi, FLAG_LOWER);
    let entry = table.get_entry(&board).unwrap();
    assert_eq!(entry.get_eval(), eval_hi);
    assert_eq!(entry.get_flag(), FLAG_LOWER);

    table.insert(&board, 0, FLAG_UPPER);
    let entry = table.get_entry(&board).unwrap();
    assert_eq!(entry.get_eval(), 0);
    assert_eq!(entry.get_flag(), FLAG_UPPER);
}
