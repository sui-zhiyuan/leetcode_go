pub fn check_powers_of_three(n: i32) -> bool {
    let mut n = n;
    while n > 0 {
        let digit = n % 3;
        if digit >= 2 {
            return false;
        }
        n /= 3;
    }
    true
}
