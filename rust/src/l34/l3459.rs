use crate::common::{Coordinate, Grid};

pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
    let grid = Grid::from(grid);
    let mut max_move = Grid::new(grid.size(), [0; 4]);

    let mut direction: [Coordinate<isize>; 4] = [
        Coordinate::new(-1, -1),
        Coordinate::new(-1, 1),
        Coordinate::new(1, 1),
        Coordinate::new(1, -1),
    ];

    for x in 0..grid.size().x {
        for y in 0..grid.size().y {
            if grid[(x, y)] == 1 {
                max_move[(x, y)] = [1, 1, 1, 1];
            }
        }
    }

    for x in 0..grid.size().x {
        for y in 0..grid.size().y {
            for (i, d) in direction.iter().enumerate().filter(|(_, d)| d.x > 0) {
                let curr: Coordinate = (x, y).into();

                if let Some(next) = curr.checked_add_signed(*d) {}
            }
        }
    }

    todo!()
}
