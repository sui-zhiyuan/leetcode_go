pub fn max_increasing_cells(mat: Vec<Vec<i32>>) -> i32 {
    let m = mat.len();
    let n = mat[0].len();

    let mut cells = mat
        .iter()
        .enumerate()
        .flat_map(|(row, row_vec)| {
            row_vec
                .iter()
                .enumerate()
                .map(move |(col, &value)| Cell { row, col, value })
        })
        .collect::<Vec<_>>();

    cells.sort_by_key(|cell| cell.value);

    let mut max_row = vec![0i32; m];
    let mut max_col = vec![0i32; n];

    let mut curr = 0;
    while curr < cells.len() {
        let mut next = curr;
        while next < cells.len() && cells[next].value == cells[curr].value {
            next += 1;
        }

        let cell_v = cells[curr..next].iter().map(|cell| {
            let v= max_row[cell.row].max(max_col[cell.col]) + 1;
            (cell.row, cell.col, v)
        }).collect::<Vec<_>>();

        for (row , col , v) in cell_v {
            max_row[row] = max_row[row].max(v);
            max_col[col] = max_col[col].max(v);
        }

        curr = next;
    }

    max_row.iter().max().unwrap().to_owned()
}

struct Cell {
    row: usize,
    col: usize,
    value: i32,
}

#[cfg(test)]
mod tests {
    use crate::common;

    use super::*;

    #[test]
    fn test1() {
        let mat = common::parse_grid::<i32>("[[3,1],[3,4]]").unwrap();
        assert_eq!(max_increasing_cells(mat), 2);
    }

    #[test]
    fn test2() {
        let mat = common::parse_grid::<i32>("[[3,1,6],[-9,5,7]]").unwrap();
        assert_eq!(max_increasing_cells(mat), 4);
    }

    #[test]
    fn test3() {
        let mat = common::parse_grid::<i32>("[[1,-8],[4,4],[-3,-9]]").unwrap();
        assert_eq!(max_increasing_cells(mat), 4);
    }
}
