pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    let n = code.len();
    let (start, end) = match k {
        0 => (0, 0),
        _ if k > 0 => (1, k as usize + 1),
        _ => (n + k as usize, n),
        
    };

    let mut sum = code[start..end].iter().sum::<i32>();
    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        result.push(sum);
        sum += code[(i + end) % n] - code[(i + start) % n];
    }
    result
}
