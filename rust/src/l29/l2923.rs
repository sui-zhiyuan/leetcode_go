impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        grid.iter()
            .enumerate()
            .position(|(i, v)| v.iter().enumerate().all(|(j, &x)| i == j || x == 1))
            .unwrap() as i32
    }
}

pub struct Solution();
