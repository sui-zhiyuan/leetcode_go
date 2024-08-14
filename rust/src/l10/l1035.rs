pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut grid = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
    for ( i , v) in nums1.iter().enumerate() {
        for ( j , w) in nums2.iter().enumerate() {
            grid[i + 1][j + 1] = if v == w {
                grid[i][j] + 1
            } else {
                grid[i][j + 1].max(grid[i + 1][j])
            }
        }
    }

    grid[nums1.len()][nums2.len()]
}

