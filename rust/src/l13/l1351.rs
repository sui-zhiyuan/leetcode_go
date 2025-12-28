pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    grid.into_iter().flatten().filter(|&x| x < 0).count() as i32
}
