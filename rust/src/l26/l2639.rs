pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let mut column_width = vec![0; grid[0].len()];

    for row in grid.iter(){
        for (j , &cell) in row.iter().enumerate(){
            column_width[j] = column_width[j].max(cell.to_string().len() as i32);
        }
    }

    column_width
}

#[cfg(test)]
mod test{
    use crate::common;

    use super::*;

    #[test]
    fn test_find_column_width1(){
        let input = common::parse_grid("[[1],[22],[333]]").unwrap();
        let result = common::parser_vec::<i32>("[3]").unwrap();
        assert_eq!(result, find_column_width(input));
    }

    #[test]
    fn test_find_column_width2(){
        let input = common::parse_grid("[[-15,1,3],[15,7,12],[5,6,-2]]").unwrap();
        let result = common::parser_vec::<i32>("[3,1,2]").unwrap();
        assert_eq!(result, find_column_width(input));
    }
}