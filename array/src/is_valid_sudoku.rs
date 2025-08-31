// https://leetcode.com/problems/valid-sudoku/solutions/7139949/

use std::collections::HashSet;

fn char_is_number(ch: char) -> bool {
    match ch.to_digit(10) {
        Some(_) => return true,
        None => return false,
    }
}

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut cell_set = HashSet::new();

    // validate rows
    for row in board.iter() {
        for cell in row.iter() {
            if char_is_number(*cell) {
                if !cell_set.insert(cell) {
                    return false;
                }
            }
        }
        cell_set.clear();
    }

    // validate columns
    for i in 0..=8 {
        for j in 0..=8 {
            let cell = &board[j][i];
            if char_is_number(*cell) {
                if !cell_set.insert(cell) {
                    return false;
                }
            }
        }
        cell_set.clear();
    }

    // validate 3x3 segments
    for x in (0..=6).step_by(3) {
        for y in (0..=6).step_by(3) {
            for i in x..=x + 2 {
                for j in y..=y + 2 {
                    let cell = &board[i][j];
                    if char_is_number(*cell) {
                        if !cell_set.insert(cell) {
                            return false;
                        }
                    }
                }
            }
            cell_set.clear();
        }
    }

    return true;
}

#[cfg(test)]
mod is_valid_sudoku_tests {
    use super::*;

    #[test]
    fn test_valid_board() {
        assert_eq!(
            is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            true
        );
    }

    #[test]
    fn test_invalid_board() {
        assert_eq!(
            is_valid_sudoku(vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            false
        );
    }
}
