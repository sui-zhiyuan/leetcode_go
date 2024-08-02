pub fn reverse(x: i32) -> i32 {
    let mut result: i32 = 0;
    let mut x = x;

    while x !=0 {
        let digit = x % 10;
        x /= 10;
        let Some(new) = result.checked_mul(10) else {
            return 0;
        };
        let Some(new) = new.checked_add(digit) else {
            return 0;
        };
        result = new;
    }
    
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_reverse() {
        assert_eq!(-321 % 10, -1);
        assert_eq!(-321 / 10, -32);
    }
}
