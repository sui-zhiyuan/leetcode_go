pub struct NeighborSum {
    neighbor_sum_value: Vec::<i32>,
    diagonal_sum_value:Vec::<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NeighborSum {

    pub fn new(grid: Vec<Vec<i32>>) -> Self {
        let n = grid.len();
        let total = n * n;

        let mut value = NeighborSum{
            neighbor_sum_value: vec![0; total],
            diagonal_sum_value: vec![0; total],
        };
        
        for (i , row) in grid.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                let mut ns = 0;
                if i > 0 {
                    ns += grid[i-1][j];
                }
                if j > 0 {
                    ns += grid[i][j-1];
                }
                if i < n - 1 {
                    ns += grid[i+1][j];
                }
                if j < n - 1 {
                    ns += grid[i][j+1];
                }
                value.neighbor_sum_value[v as usize] = ns;

                let mut ds = 0;
                if i > 0 && j > 0 {
                    ds += grid[i-1][j-1];
                }
                if i < n - 1 && j < n - 1 {
                    ds += grid[i+1][j+1];
                }
                if i > 0 && j < n - 1 {
                    ds += grid[i-1][j+1];
                }
                if i < n - 1 && j > 0 {
                    ds += grid[i+1][j-1];
                }

                value.diagonal_sum_value[v as usize] = ds;
            }
        }

        value
    }
    
    pub fn adjacent_sum(&self, value: i32) -> i32 {
        self.neighbor_sum_value[value as usize]
    }
    
    pub fn diagonal_sum(&self, value: i32) -> i32 {
        self.diagonal_sum_value[value as usize]
    }
}

