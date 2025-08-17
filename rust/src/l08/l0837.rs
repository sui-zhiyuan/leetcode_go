pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
    let n = n as usize;
    let k = k as usize;
    let max_pts = max_pts as usize;

    if max_pts <= n - k + 1 {
        return 1f64;
    }
    let mut prob = vec![0f64; k + max_pts];

    for i in k..=n {
        prob[i] = 1f64
    }

    let mut sum = 0f64;
    for i in k..(k + max_pts) {
        sum += prob[i]
    }

    for i in (0..k).rev() {
        prob[i] = sum / (max_pts as f64);
        sum -= prob[i + max_pts];
        sum += prob[i]
    }

    prob[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!((0.73278f64 - new21_game(21, 17, 10)).abs() < 0.00005)
    }
}
