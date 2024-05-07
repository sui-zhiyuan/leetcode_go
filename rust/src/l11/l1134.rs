pub fn is_armstrong(n: i32) -> bool {
    n < 10 || ([
        153, 370, 371, 407, 1634, 8208, 9474, 54748, 92727, 93084, 548834, 1741725, 4210818,
        9800817, 9926315, 24678050, 24678051, 88593477, 146511208, 472335975, 534494836, 912985153,
    ])
    .iter()
    .any(|&x| x == n)
}

// debug and test

pub fn calc_is_armstrong(n: i32) -> bool {
    let mut sum = 0;
    let mut vn: i32 = n;
    let nd = (n as f64).log10() as u32 + 1;

    while vn > 0 {
        let digit = vn % 10;
        sum += digit.pow(nd);
        vn /= 10;
        if sum > n {
            return false;
        }
    }

    sum == n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        for i in 0..=100 {
            if calc_is_armstrong(i) {
                println!("{}", i);
            }
        }
    }

    #[test]
    fn test2() {
        assert!(is_armstrong(153));
    }
}
