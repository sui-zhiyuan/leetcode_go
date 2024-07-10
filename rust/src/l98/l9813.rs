use std::ops::Add;

pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
    let mut counts = vec![vec![Sum::default(); grid[0].len()]; grid.len()];
    let n = grid[0].len();

    let mut result = 0; 
    for (i , row) in grid.into_iter().enumerate() {
        let mut sum_row = vec![Sum::default() ; n];
        for (j, v) in row.into_iter().enumerate() {
            let prev = if i == 0 { Sum::default() } else { counts[i-1][j] };
            let prev2 = if j == 0 { Sum::default() } else { sum_row[j-1] };

            let curr = if v == '.' {
                Sum {
                    count_x: 0,
                    count_y: 0,
                }
            } else if v == 'X' {
                Sum {
                    count_x: 1,
                    count_y: 0,
                }
            } else if v == 'Y' {
                Sum {
                    count_x: 0,
                    count_y: 1,
                }
            } else {
                panic!("invalid value")
            };

            sum_row[j] = prev2 + curr;
            counts[i][j] = prev + prev2 + curr;

            if counts[i][j].count_x > 0 && counts[i][j].count_y > 0 && counts[i][j].count_x == counts[i][j].count_y{
                result += 1;
            }
        }
    }
    result
}

#[derive(Clone , Debug , Copy , Default)]
struct Sum {
    count_x: i32,
    count_y: i32,
}

impl Add for Sum {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Sum {
            count_x: self.count_x + other.count_x,
            count_y: self.count_y + other.count_y,
        }
    }
}


