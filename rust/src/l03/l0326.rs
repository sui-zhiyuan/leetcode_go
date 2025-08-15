pub fn is_power_of_three(n: i32) -> bool {
    const MAX_3_POW: i32 = get_max_power();
    n > 0 && MAX_3_POW % n == 0
}

const fn get_max_power() -> i32 {
    let mut v = 3i32;
    loop {
        let Some(next) = v.checked_mul(3) else {
            return v;
        };
        v = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(is_power_of_three(27))
    }
}
