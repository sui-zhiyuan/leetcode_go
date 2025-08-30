use crate::common::{Coordinate, Grid};

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let board = Grid::from(board);

    for x in 0..board.size().x {
        let mut find = [false; 10];
        for y in 0..board.size().y {
            if !check_find(board[(x, y)], &mut find) {
                return false;
            }
        }
    }

    for y in 0..board.size().y {
        let mut find = [false; 10];
        for x in 0..board.size().x {
            if !check_find(board[(x, y)], &mut find) {
                return false;
            }
        }
    }

    let starts = [
        (0, 0),
        (0, 3),
        (0, 6),
        (3, 0),
        (3, 3),
        (3, 6),
        (6, 0),
        (6, 3),
        (6, 6),
    ];

    let steps = [
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 0),
        (1, 1),
        (1, 2),
        (2, 0),
        (2, 1),
        (2, 2),
    ];

    for start in starts {
        let mut find = [false; 10];
        for step in steps {
            let start: Coordinate = start.into();
            let step: Coordinate = step.into();

            if !check_find(board[start + step], &mut find) {
                return false;
            }
        }
        dbg!("=== new round ===");
    }

    true
}

fn check_find(value: char, find: &mut [bool; 10]) -> bool {
    match value {
        '.' => true,
        v => {
            let v: usize = v.to_string().parse().expect("not a number");
            if find[v] {
                return false;
            }
            find[v] = true;
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let border = [
            [".", ".", ".", ".", "5", ".", ".", "1", "."],
            [".", "4", ".", "3", ".", ".", ".", ".", "."],
            [".", ".", ".", ".", ".", "3", ".", ".", "1"],
            ["8", ".", ".", ".", ".", ".", ".", "2", "."],
            [".", ".", "2", ".", "7", ".", ".", ".", "."],
            [".", "1", "5", ".", ".", ".", ".", ".", "."],
            [".", ".", ".", ".", ".", "2", ".", ".", "."],
            [".", "2", ".", "9", ".", ".", ".", ".", "."],
            [".", ".", "4", ".", ".", ".", ".", ".", "."],
        ];

        let border: Vec<Vec<char>> = border
            .into_iter()
            .map(|row| row.into_iter().map(|v| v.chars().next().unwrap()).collect())
            .collect();

        assert!(!is_valid_sudoku(border));
    }
}
