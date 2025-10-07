use crate::common::{Coordinate, Grid};
use std::fmt::{Debug, Formatter};

pub fn solve_sudoku(#[allow(clippy::ptr_arg)] outer_board: &mut Vec<Vec<char>>) {
    let mut board = Grid::<char>::from(outer_board.clone()).map(|v| match v {
        '.' => None,
        c @ '1'..='9' => Some(c.to_string().parse::<u8>().unwrap()),
        _ => panic!("invalid input"),
    });

    let mut status = Status::new();

    for x in 0..board.size().x {
        for y in 0..board.size().y {
            let Some(v) = board[(x, y)] else {
                continue;
            };
            status.apply((x, y).into(), v);
        }
    }

    assert!(find_solution(&mut board, &status));

    for x in 0..board.size().x {
        for y in 0..board.size().y {
            outer_board[x][y] = board[(x, y)].unwrap().to_string().chars().next().unwrap();
        }
    }
}

fn find_solution(board: &mut Grid<Option<u8>>, status: &Status) -> bool {
    let mut min_choice = usize::MAX;
    let mut min_coor = Coordinate { x: 0, y: 0 };

    let size = board.size();

    for x in 0..size.x {
        for y in 0..size.y {
            if board[(x, y)].is_some() {
                continue;
            }
            let c = status.choices((x, y).into());
            if c.is_empty() {
                return false;
            } else if c.len() < min_choice {
                min_choice = c.len();
                min_coor = (x, y).into();
            }
        }
    }

    if min_choice == usize::MAX {
        return true;
    }

    let choices = status.choices(min_coor);

    for &c in choices.iter() {
        assert!(board[min_coor].is_none());
        board[min_coor] = Some(c);
        let mut new_status = *status;
        new_status.apply(min_coor, c);

        // println!("chose {c} at {min_coor:?} from {:?}", &choices);
        // debug_all(board, status);
        if find_solution(board, &new_status) {
            return true;
        }
        board[min_coor] = None;
    }
    false
}

#[derive(Clone, Copy)]
struct Status {
    row: [u16; 9],
    col: [u16; 9],
    seq: [u16; 9],
}

impl Status {
    fn new() -> Self {
        Status {
            row: [0x03feu16; 9],
            col: [0x03feu16; 9],
            seq: [0x03feu16; 9],
        }
    }

    fn apply(&mut self, coor: Coordinate, value: u8) {
        let value_mask = !(1 << value);
        self.row[coor.x] &= value_mask;
        self.col[coor.y] &= value_mask;
        let seq_index = coor.x / 3 * 3 + coor.y / 3;
        self.seq[seq_index] &= value_mask;
    }

    fn choices(self, coor: Coordinate) -> Vec<u8> {
        let seq_index = coor.x / 3 * 3 + coor.y / 3;

        if coor == (6, 2).into() {
            println!();
        }

        let choices = self.row[coor.x] & self.col[coor.y] & self.seq[seq_index];
        let mut ret = Vec::new();
        for i in 1..=9 {
            let v = 1 << i;
            if choices & v > 0 {
                ret.push(i as u8);
            }
        }

        ret
    }
}

// debug and test
#[allow(unused)]
fn debug_all(grid: &Grid<Option<u8>>, status: &Status) {
    debug_grid(grid);
    println!("{status:?}");
}

impl Debug for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let target = [("row", self.row), ("col", self.col), ("seq", self.seq)];
        for t in target {
            writeln!(f, "{}", t.0)?;
            for (i, &v) in t.1.iter().enumerate() {
                write!(f, "{i}: ")?;
                for j in 1..=9 {
                    if (1 << j) & v > 0 {
                        write!(f, "{j}")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
                writeln!(f)?
            }
        }
        Ok(())
    }
}

#[allow(unused)]
fn debug_grid(grid: &Grid<Option<u8>>) {
    for x in 0..grid.size().x {
        for y in 0..grid.size().y {
            match grid[(x, y)] {
                None => print!("."),
                Some(c) => print!("{}", c),
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let board = [
            ["5", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ];

        let expect = [
            ["5", "3", "4", "6", "7", "8", "9", "1", "2"],
            ["6", "7", "2", "1", "9", "5", "3", "4", "8"],
            ["1", "9", "8", "3", "4", "2", "5", "6", "7"],
            ["8", "5", "9", "7", "6", "1", "4", "2", "3"],
            ["4", "2", "6", "8", "5", "3", "7", "9", "1"],
            ["7", "1", "3", "9", "2", "4", "8", "5", "6"],
            ["9", "6", "1", "5", "3", "7", "2", "8", "4"],
            ["2", "8", "7", "4", "1", "9", "6", "3", "5"],
            ["3", "4", "5", "2", "8", "6", "1", "7", "9"],
        ];

        let mut board = to_input(board);
        let expect = to_input(expect);

        solve_sudoku(&mut board);
        assert_eq!(expect, board);
    }

    fn to_input(v: [[&'static str; 9]; 9]) -> Vec<Vec<char>> {
        v.into_iter()
            .map(|r| {
                r.into_iter()
                    .map(|v| v.chars().next().unwrap())
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<_>>()
    }
}
