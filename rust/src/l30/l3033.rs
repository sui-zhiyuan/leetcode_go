pub fn modified_matrix(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut max = vec![0 ; matrix[0].len()];

    for row in matrix.iter() {
        for ( j , &v) in row.iter().enumerate() {
            max[j] = max[j].max(v);
        }
    }

    for row in matrix.iter_mut(){
        for (j , v) in row.iter_mut().enumerate(){
            if *v == -1 {
                *v = max[j];
            }
        }
    }

    matrix
}