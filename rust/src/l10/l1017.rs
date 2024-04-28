pub fn base_neg2(mut n: i32) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let mut normal = Vec::new();
    while n > 0 {
        normal.push(n & 1);
        n >>= 1;
    }

    let mut i = 0;
    let mut positive = true;
    let mut carry = 0;
    let mut result = Vec::new();
    while i < normal.len() || carry > 0 {
        if i < normal.len() {
            carry += normal[i];
        }
        if carry % 2 == 0 {
            result.push(0);
        } else {
            result.push(1);
            if positive {
                carry -= 1;
            } else {
                carry += 1;
            }
        }

        carry /= 2;
        i += 1;
        positive = !positive
    }

    result.iter().rev().map(|x| x.to_string()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!("110", base_neg2(2));
        assert_eq!("111", base_neg2(3));
        assert_eq!("100", base_neg2(4));
    }
}
