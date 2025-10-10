use crate::common::{Coordinate, Grid};
use std::collections::VecDeque;

pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
    let grid = Grid::from(grid);
    let mut arrive = Grid::<i32>::new(grid.size(), i32::MAX);
    let mut queue = VecDeque::<Coordinate>::new();

    const DIRECTIONS: [Coordinate<isize>; 4] = [
        Coordinate::<isize>::new(-1, 0),
        Coordinate::<isize>::new(0, 1),
        Coordinate::<isize>::new(1, 0),
        Coordinate::<isize>::new(0, -1),
    ];

    arrive[(0, 0)] = grid[(0, 0)];
    queue.push_back((0, 0).into());

    while let Some(curr) = queue.pop_front() {
        for &d in DIRECTIONS.iter() {
            let Some(next) = curr.checked_add_signed(d) else {
                continue;
            };
            if !next.inside(grid.size()) {
                continue;
            }

            let new_elevation = arrive[curr].max(grid[next]);
            if arrive[next] > new_elevation {
                arrive[next] = new_elevation;
                queue.push_back(next);
            }
            // print(&arrive, curr, next);
        }
    }

    arrive[(grid.size().x - 1, grid.size().y - 1)]
}

// debug and test
#[allow(unused)]
fn print(arrive: &Grid<i32>, curr: Coordinate, next: Coordinate) {
    println!("{curr:?} {next:?}");
    for x in 0..arrive.size().x {
        for y in 0..arrive.size().y {
            print!("{:3} ", arrive[(x, y)]);
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{common, parameter_tests};

    struct Case {
        grid: &'static str,
        expect: i32,
    }

    fn test(c: Case) {
        let grid = common::parse_grid::<i32>(c.grid).unwrap();
        assert_eq!(c.expect, swim_in_water(grid));
    }

    parameter_tests! {
        test,
        (case1: Case {
            grid: "[[0,2],[1,3]]",
            expect: 3,
        }),
        (case2: Case {
            grid: "[[0,1,2,3,4],[24,23,22,21,5],[12,13,14,15,16],[11,17,18,19,20],[10,9,8,7,6]]",
            expect: 16,
        }),
    }
}
