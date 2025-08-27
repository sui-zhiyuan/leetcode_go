use crate::common::Grid;

pub fn max_coins(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut max_coins = Grid::new((n, n), 0);

    let get_prev = |i: usize| -> i32 {
        if i == 0 {
            1
        } else {
            nums[i - 1]
        }
    };
    let get_next = |i: usize| -> i32 {
        if i == n - 1 {
            1
        } else {
            nums[i + 1]
        }
    };

    for l in 0..n {
        for i in 0..n - l {
            let j = i + l;
            if l == 0 {
                max_coins[(i, j)] = get_prev(i) * nums[i] * get_next(i);
                continue;
            }
            for k in i..=j {
                max_coins[(i, j)] = max_coins[(i, j)].max(
                    get_prev(i) * nums[k] * get_next(j)
                        + (if k > i { max_coins[(i, k - 1)] } else { 0 })
                        + (if k < j { max_coins[(k + 1, j)] } else { 0 }),
                );
            }
        }
    }
    max_coins[(0, n - 1)]
}
