// Connect4EngineRust, a strong solver for the connect-4 board game.
// Copyright (C) 2023 Derick Tseng
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use connect4engine::board::Board;
use connect4engine::transpositiontable::MAX_TABLE_SIZE;
use connect4engine::transpositiontable::TranspositionTable;
use connect4engine::transpositiontable::FLAG_UPPER;
use connect4engine::transpositiontable::FLAG_LOWER;

#[test]
fn test_insert_get() {
    let board = Board::new_position("44444752222436656566263375515127171771313").unwrap();
    let eval_hi = 5;

    let mut table = TranspositionTable::new();
    let entry = table.get_entry(&board);
    assert!(entry.is_none());

    table.insert(&board, eval_hi, FLAG_UPPER, 15, 3);
    let entry = table.get_entry(&board);

    assert!(entry.is_some());
    let entry = entry.unwrap();

    assert_eq!(entry.get_eval(), eval_hi);
    assert_eq!(entry.get_flag(), FLAG_UPPER);
    assert_eq!(entry.get_depth(), 15);
    assert_eq!(entry.get_mv(), 3);
}

#[test]
fn test_insert_get_negative() {
    let board = Board::new_position("44444752222436656566263375515127171771313").unwrap();
    let eval_hi = -1;

    let mut table = TranspositionTable::new();
    let entry = table.get_entry(&board);
    assert!(entry.is_none());

    table.insert(&board, eval_hi, FLAG_UPPER, 15, 3);
    let entry = table.get_entry(&board);
    assert!(entry.is_some());
    let entry = entry.unwrap();
    assert_eq!(entry.get_eval(), eval_hi);
    assert_eq!(entry.get_flag(), FLAG_UPPER);
}

#[test]
fn test_collision() {
    let mut table = TranspositionTable::new();
    let key1: u64 = 3;
    let key2 = MAX_TABLE_SIZE as u64 + key1;
    let entry = table.get_entry_with_key(key1);
    assert!(entry.is_none());

    // expect to have entry1
    let depth1 = 15;
    let eval1 = 13;
    let flag1 = FLAG_LOWER;
    let mv1 = 3;
    table.insert_with_key(key1, eval1, flag1, depth1, mv1);
    let entry = table.get_entry_with_key(key1);
    assert!(entry.is_some());

    let entry = entry.unwrap();
    assert_eq!(entry.get_eval(), eval1);
    assert_eq!(entry.get_flag(), flag1);
    assert_eq!(entry.get_depth(), depth1);
    assert_eq!(entry.get_mv(), mv1);

    // expect to have entry2
    let depth2 = 16;
    let eval2 = 14;
    let flag2 = FLAG_UPPER;
    let mv2 = 4;
    table.insert_with_key(key2, eval2, flag2, depth2, mv2);
    let entry = table.get_entry_with_key(key2);
    assert!(entry.is_some());

    let entry = entry.unwrap();
    assert_eq!(entry.get_eval(), eval2);
    assert_eq!(entry.get_flag(), flag2);
    assert_eq!(entry.get_depth(), depth2);
    assert_eq!(entry.get_mv(), mv2);

    // expect entry1 to still be around.
    let entry = table.get_entry_with_key(key1);
    assert!(entry.is_some());

    let entry = entry.unwrap();
    assert_eq!(entry.get_eval(), eval1);
    assert_eq!(entry.get_flag(), flag1);
    assert_eq!(entry.get_depth(), depth1);
    assert_eq!(entry.get_mv(), mv1);
}
