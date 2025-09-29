pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
    let mut ans = 0;

    for q in queries {
        ans += (sum_to_n(q[1]) - sum_to_n(q[0] - 1) + 1) / 2;
    }

    ans
}

fn sum_to_n(n: i32) -> i64 {
    let m = (size_of_val(&n) * 8) as i64 - n.leading_zeros() as i64;
    let mut ret = 0;
    for i in 1..m {
        ret += ((i + 1) / 2) << (i - 1);
    }

    ret + (m + 1) / 2 * (n as i64 + 1 - (1 << (m as usize - 1)))
}
