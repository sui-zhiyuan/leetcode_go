pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = mat.len();
    let m = mat[0].len();
    let mut mat = mat;

    for i in 0..n {
        let mut pos = (i , 0);
        let mut values = Vec::new();
        while pos.0 < n && pos.1 <m {
            values.push(mat[pos.0][pos.1]);
            pos.0 += 1;
            pos.1 += 1;
        }
        values.sort();
        for (k , &v) in values.iter().enumerate(){
            mat[i +k][k] = v;
        }
    }

    for j in 1..m {
        let mut pos = (0 , j);
        let mut values = Vec::new();
        while pos.0 < n && pos.1 <m {
            values.push(mat[pos.0][pos.1]);
            pos.0 += 1;
            pos.1 += 1;
        }
        values.sort();
        for (k , &v) in values.iter().enumerate(){
            mat[k][j + k] = v;
        }
    }
    
    mat
}


#[cfg(test)]
mod test{
    use crate::common;

    use super::*;

    #[test]
    fn test1(){
        let mat = common::parse_grid::<i32>("[[3,3,1,1],[2,2,1,2],[1,1,1,2]]").unwrap();
        let result = common::parse_grid::<i32>("[[1,1,1,1],[1,2,2,2],[1,2,3,3]]").unwrap();

        assert_eq!(result, diagonal_sort(mat));
    }

    #[test]
    fn test2(){
        let mat = common::parse_grid::<i32>("[[11,25,66,1,69,7],[23,55,17,45,15,52],[75,31,36,44,58,8],[22,27,33,25,68,4],[84,28,14,11,5,50]]").unwrap();
        let result = common::parse_grid::<i32>("[[5,17,4,1,52,7],[11,11,25,45,8,69],[14,23,25,44,58,15],[22,27,31,36,50,66],[84,28,75,33,55,68]]").unwrap();

        assert_eq!(result, diagonal_sort(mat));
    }
}