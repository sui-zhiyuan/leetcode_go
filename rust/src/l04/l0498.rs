pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let m = mat.len();
    let n = mat[0].len();

    let mut result = Vec::new();
    for i in 0..(m + n - 1) {
        let range: Box<dyn Iterator<Item = usize>> = if i % 2 == 0 {
            Box::new(0..n)
        } else {
            Box::new((0..n).rev())
        };

        for col in range {
            let row = i - col;

            if col >= n || row >= m {
                continue;
            }
            result.push(mat[row][col])
        }
    }

    result
}
