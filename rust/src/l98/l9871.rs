pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let n = s.len();
    let mut sum = vec![(0, 0); n + 1];
    for (i, c) in s.iter().enumerate() {
        sum[i + 1] = match c {
            '1' => (sum[i].0 + 1, sum[i].1),
            '0' => (sum[i].0, sum[i].1 + 1),
            _ => panic!("invalid char"),
        };
    }
    let mut count = 0;
    for i in 0..n {
        for j in i + 1..=n {
            if sum[j].0 - sum[i].0 <= k || sum[j].1 - sum[i].1 <= k {
                count += 1;
            }
        }
    }

    count
}
