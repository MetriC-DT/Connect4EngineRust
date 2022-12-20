use connect4engine::board::Board;
use connect4engine::transpositiontable::FLAG_EXACT;
use connect4engine::transpositiontable::TranspositionTable;
use connect4engine::transpositiontable::FLAG_UPPER;
use connect4engine::transpositiontable::FLAG_LOWER;

#[test]
fn test_insert_get() {
    let board = Board::new_position("44444752222436656566263375515127171771313");
    let eval_hi = 5;

    let mut table = TranspositionTable::new();
    assert!(table.get_entry(&board).is_none());

    table.insert(&board, eval_hi, FLAG_UPPER, 4);
    let entry = table.get_entry(&board).unwrap();
    assert_eq!(entry.get_eval(), eval_hi);
    assert_eq!(entry.get_flag(), FLAG_UPPER);
    assert_eq!(entry.get_move(), 4);
}

#[test]
fn test_insert_get_negative() {
    let board = Board::new_position("44444752222436656566263375515127171771313");
    let eval_hi = -1;

    let mut table = TranspositionTable::new();
    assert!(table.get_entry(&board).is_none());

    table.insert(&board, eval_hi, FLAG_UPPER, 3);
    let entry = table.get_entry(&board).unwrap();
    assert_eq!(entry.get_eval(), eval_hi);
    assert_eq!(entry.get_flag(), FLAG_UPPER);
    assert_eq!(entry.get_move(), 3);
}

#[test]
fn test_evict() {
    let board = Board::new_position("44444752222436656566263375515127171771313");
    let eval_hi = 10;

    let mut table = TranspositionTable::new();
    assert!(table.get_entry(&board).is_none());

    table.insert(&board, eval_hi, FLAG_LOWER, 5);
    let entry = table.get_entry(&board).unwrap();
    assert_eq!(entry.get_eval(), eval_hi);
    assert_eq!(entry.get_flag(), FLAG_LOWER);
    assert_eq!(entry.get_move(), 5);

    table.insert(&board, 0, FLAG_EXACT, 0);
    let entry = table.get_entry(&board).unwrap();
    assert_eq!(entry.get_eval(), 0);
    assert_eq!(entry.get_flag(), FLAG_EXACT);
    assert_eq!(entry.get_move(), 0);
}
