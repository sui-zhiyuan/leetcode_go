pub fn remove_zeros(n: i64) -> i64 {
    let mut n  = n;
    let mut result = 0i64;
    let mut dig = 1;

    while n > 0 {
        let d  = n % 10;
        n /= 10;
        if d > 0 {
            result += d * dig;
            dig *= 10;
        }
    }
    
    result
}