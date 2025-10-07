use crate::common::{Coordinate, Grid};

const DIRECTIONS: [Coordinate<isize>; 4] = [
    Coordinate::new(-1, -1),
    Coordinate::new(-1, 1),
    Coordinate::new(1, 1),
    Coordinate::new(1, -1),
];

pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
    let grid = Grid::from(grid);
    let mut max_move = Grid::new(grid.size(), [0; 4]);

    for c in grid.size().iter() {
        if grid[c] == 1 {
            max_move[c] = [1, 1, 1, 1];
        }
    }

    for c in grid.size().iter() {
        for (i, d) in DIRECTIONS.iter().enumerate().filter(|(_, d)| d.x > 0) {
            update_length(&grid, &mut max_move, (i, d), c)
        }
    }

    for c in grid.size().iter().rev() {
        for (i, d) in DIRECTIONS.iter().enumerate().filter(|(_, d)| d.x < 0) {
            update_length(&grid, &mut max_move, (i, d), c)
        }
    }

    // println!("first round");
    // print_move(&max_move);

    for c in grid.size().iter() {
        let mut turned = [0; 4];
        for (i, v) in turned.iter_mut().enumerate() {
            *v = if i == 0 {
                max_move[c][3]
            } else {
                max_move[c][i - 1]
            }
        }
        max_move[c] = turned;
    }

    // println!("turned");
    // print_move(&max_move);

    for c in grid.size().iter() {
        for (i, d) in DIRECTIONS.iter().enumerate().filter(|(_, d)| d.x > 0) {
            update_length(&grid, &mut max_move, (i, d), c)
        }
    }

    for c in grid.size().iter().rev() {
        for (i, d) in DIRECTIONS.iter().enumerate().filter(|(_, d)| d.x < 0) {
            update_length(&grid, &mut max_move, (i, d), c)
        }
    }

    // println!("second round");
    // print_move(&max_move);

    max_move.iter().flatten().max().copied().unwrap()
}

fn update_length(
    grid: &Grid<i32>,
    max_move: &mut Grid<[i32; 4]>,
    (di, d): (usize, &Coordinate<isize>),
    curr: Coordinate,
) {
    if max_move[curr][di] <= 0 {
        return;
    }
    let Some(next) = curr.checked_add_signed(*d) else {
        return;
    };
    if !next.inside(grid.size()) {
        return;
    }

    let mut curr_v = grid[curr];
    let next_v = grid[next];
    if curr_v == 1 {
        curr_v = 0;
    }

    if curr_v + next_v == 2 {
        max_move[next][di] = max_move[next][di].max(max_move[curr][di] + 1);
    }
}

// debug and test
#[allow(dead_code)]
fn print_move(max_move: &Grid<[i32; 4]>) {
    let sep_row = str::repeat("-", max_move.size().y * 4 + 1);
    for x in 0..max_move.size().x {
        println!("{}", sep_row);
        for y in 0..max_move.size().y {
            let m = max_move[(x, y)];
            print!("|{},{}", m[0], m[1]);
        }
        println!("|");
        for y in 0..max_move.size().y {
            let m = max_move[(x, y)];
            print!("|{},{}", m[3], m[2]);
        }
        println!("|");
    }
    println!("{}", sep_row);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::parse_grid;
    use crate::parameter_tests;

    struct Case {
        grid: &'static str,
        expect: i32,
    }

    fn test(c: Case) {
        let grid = parse_grid::<i32>(c.grid).unwrap();
        assert_eq!(c.expect, len_of_v_diagonal(grid));
    }

    parameter_tests! {
        test,
        (case1: Case {
            grid: "[[2,2,1,2,2],[2,0,2,2,0],[2,0,1,1,0],[1,0,2,2,2],[2,0,0,2,2]]",
            expect: 5,
        }),
        (case2: Case {
            grid: "[[2,2,2,2,2],[2,0,2,2,0],[2,0,1,1,0],[1,0,2,2,2],[2,0,0,2,2]]",
            expect: 4,
        }),
        (case3: Case {
            grid: "[[1,2,2,2,2],[2,2,2,2,0],[2,0,0,0,0],[0,0,2,2,2],[2,0,0,2,0]]",
            expect: 5,
        }),
        (case4: Case {
            grid: "[[1]]",
            expect: 1,
        }),
        (case5: Case {
            grid: "[[0,0,2,0,0,2,1,1,2],[2,2,2,1,2,0,2,2,2],[0,0,2,0,0,1,2,1,0],[2,0,2,2,1,1,2,2,2],[0,0,2,0,0,0,1,1,2],[0,0,1,0,2,1,1,2,2],[0,0,2,0,0,1,0,0,0]]",
            expect: 8,
        }),
    }
}
