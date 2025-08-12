pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let queries = queries
        .into_iter()
        .map(|v| (v[0], v[1]))
        .collect::<Vec<_>>();

    // power of value , using sum of power to save memory
    let mut powers = Vec::new();

    for i in 0..31 {
        if n & (1 << i) > 0 {
            powers.push(i)
        }
    }

    let mut sum_power = Vec::<i32>::new();
    sum_power.push(0);

    for p in powers {
        let prev = sum_power.last().copied().unwrap();
        sum_power.push(prev + p);
    }

    let mut result = Vec::new();

    for v in queries {
        // [left..<right]
        let left = v.0 as usize;
        let right = (v.1 + 1) as usize;
        // [left..<right]
        result.push(power_2(sum_power[right] - sum_power[left]))
    }

    result
}

fn power_2(v: i32) -> i32 {
    let mut result = 1i64;
    const MOD: i64 = 1_000_000_007;
    for i in (0..31).rev() {
        result = (result * result) % MOD;
        if v & (1 << i) > 0 {
            result = result * 2 % MOD
        }
    }

    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 15;
        let queries = [[0, 1], [2, 2], [0, 3]]
            .into_iter()
            .map(|v| v.to_vec())
            .collect::<Vec<_>>();
        let result = vec![2, 4, 64];
        assert_eq!(result, product_queries(n, queries))
    }
}
