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

use connect4engine::moves::{DEFAULT_ORDER, Moves};
use connect4engine::board::{HEIGHT, Board, WIDTH, BOTTOM_ROW_MASK, COLUMN_MASK, COUNTS_PER_COL};

fn test_winning_line(line: &str) {
    let mut b = Board::new();
    // play up until the last move, but not the last one.
    for c in line[..line.len() - 1].chars() {
        let col: u8 = c.to_digit(10).unwrap() as u8 - 1;
        assert!(b.add(col).is_ok());
        assert!(!b.is_filled());
        assert!(!b.has_winner());
    }

    let col: char = line.as_bytes()[line.len() - 1] as char;
    let col: u8 = col.to_digit(10).unwrap() as u8 - 1;
    assert!(b.add(col).is_ok());
    assert!(b.has_winner());
}

fn test_pre_win(line: &str, one_bit_set: bool) {
    let b = Board::new_position(line).unwrap();
    let possible = b.possible_moves();
    let w_mvs = b.player_win_moves(possible);
    assert_ne!(w_mvs, 0);
    assert_eq!(Board::at_most_one_bit_set(w_mvs), one_bit_set);
}

#[test]
fn test_board_add() {
    let mut b = Board::new();
    let col: u8 = 3;
    for i in 0..HEIGHT {
        assert_eq!(b.get_height(col), i);
        assert!(b.add(col).is_ok());
    }

    // cannot add when height max is reached.
    assert!(b.add(col).is_err());
    assert_eq!(b.get_height(col), HEIGHT);
}

#[test]
fn test_board_revert() {
    let mut b = Board::new_position("16357157437461355316457465722").unwrap();
    let possible = b.possible_moves();
    let orig_pos_key = b.get_unique_position_key();
    for (mv, _c) in Moves::new(possible) {
        b.play(mv);
        let new_key = b.get_unique_position_key();
        assert_ne!(orig_pos_key, new_key);
        b.revert(mv);
        let revert_key = b.get_unique_position_key();
        assert_eq!(orig_pos_key, revert_key);
    }
}


#[test]
fn test_not_win_1() {
    let line = "16357157437461355316457465722";
    let b = Board::new_position(line).unwrap();
    let possible = b.possible_moves();
    assert_eq!(b.player_win_moves(possible), 0);
}

#[test]
fn test_not_win_2() {
    let line = "4444475222243665656626337551512717177131";
    let b = Board::new_position(line).unwrap();
    let possible = b.possible_moves();
    println!("{}", b);
    assert_ne!(b.player_win_moves(possible), 0);
}

#[test]
fn test_pre_win_1() {
    let line = "323232";
    let b = Board::new_position(line).unwrap();
    let possible = b.possible_moves();
    let w_mvs = b.player_win_moves(possible);
    assert_ne!(w_mvs, 0);
    assert!(Board::at_most_one_bit_set(w_mvs));
}

#[test]
fn test_essential_1() {
    let line = "444444326552322556";
    let b = Board::new_position(line).unwrap();
    let possible = b.possible_moves();
    let (essential, _) = b.opp_win_moves(possible);
    assert_ne!(essential, 0);

    // should be in column 6.
    assert_eq!(essential, BOTTOM_ROW_MASK & (COLUMN_MASK << (6 * COUNTS_PER_COL)))
}

#[test]
fn test_pre_win_singles() {
    // only 1 line set
    let lines = [
        "112233", // win to right
        "776655", // win to left
        "113344", // win in between left
        "112244", // win in between right
        "4444443265523225563" // win on diagonal
    ];

    for line in lines {
        test_pre_win(line, true);
    }
}

#[test]
fn test_pre_win_multiples() {
    let lines = [
        "223344" // win on left and right
    ];
    for line in lines {
        test_pre_win(line, false);
    }
}

#[test]
fn test_win_vertical() {
    let line = "3232323";
    test_winning_line(line);
}

#[test]
fn test_win_horizontal() {
    let line = "1122334";
    test_winning_line(line);
}

#[test]
fn test_valid_moves() {
    let mut b = Board::new();
    let moves = Moves::new(b.possible_moves());
    assert_eq!(moves.clone().count(), WIDTH as usize);

    // has all columns available
    for ((mv, c), col) in moves.clone().zip(DEFAULT_ORDER) {
        assert_eq!(Board::pos_to_col(mv), col);
        assert_eq!(col, c);
    }

    // fills the 3rd and 6th columns
    for _ in 0..HEIGHT {
        b.add(3).unwrap();
        b.add(6).unwrap();
    }

    let moves = Moves::new(b.possible_moves());
    assert_eq!(moves.count(), (WIDTH - 2) as usize);

    // has these columns
    let has_moves = [0, 1, 2, 4, 5];
    for (mv, c) in Moves::new(b.possible_moves()) {
        let col = Board::pos_to_col(mv);
        assert_eq!(col, c);
        assert!(has_moves.contains(&col));
    }

    // does not have these columns
    let not_has_moves = [3, 6];
    for (mv, c) in Moves::new(b.possible_moves()) {
        let col = Board::pos_to_col(mv);
        assert_eq!(col, c);
        assert!(!not_has_moves.contains(&col));
    }
}

#[test]
fn test_filled() {
    let mut b = Board::new();
    for _ in 0..HEIGHT {
        for col in 0..WIDTH {
            assert!(b.add(col).is_ok());
        }
    }

    assert!(b.is_filled());
}

#[test]
fn test_not_filled() {
    let mut b = Board::new();
    for col in 0..WIDTH {
        b.add(col).unwrap();
    }

    assert!(!b.is_filled());
}

#[test]
fn test_unique_position_key() {
    let mut b = Board::new();
    let moves = "333336411113255454551522644040160606602022";
    let mut seen_keys = Vec::new();

    // simulates a game to check if keys collide
    for col in moves.chars() {
        let unique_position_key = b.get_unique_position_key();
        assert!(!seen_keys.contains(&unique_position_key));
        seen_keys.push(unique_position_key);

        let col = col.to_digit(10).unwrap();
        b.add(col as u8).unwrap();
    }

    // checks last one
    let unique_position_key = b.get_unique_position_key();
    assert!(!seen_keys.contains(&unique_position_key));
}
